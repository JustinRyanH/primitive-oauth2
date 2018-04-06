//! Client for RFC 6749, a.k.a, OAuth 2.0 Framework
pub mod authenticator;
pub mod mock_client;
pub mod mock_server;
pub mod params;
pub mod storage;

#[cfg(test)]
pub mod authenticator_spec;
#[cfg(test)]
pub mod mock_client_spec;
#[cfg(test)]
pub mod params_spec;

use errors::{Error, ErrorKind, Result};
use futures::future::Future;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Hash)]
pub enum AccessType {
    Implicit,
    Grant,
}

impl AccessType {
    pub fn get_response_type(&self) -> &'static str {
        match self {
            &AccessType::Implicit => "token",
            &AccessType::Grant => "code",
        }
    }
}

#[doc(hidden)]
/// Convenience trait that convert `Future` object into `Boxed` future
pub trait AsyncPacker<I, E>: Sized {
    fn pack(self) -> Box<Future<Item = I, Error = E> + Send>;
}

impl<F, I, E> AsyncPacker<I, E> for F
where
    F: Future<Item = I, Error = E> + 'static + Send,
    E: Into<Error> + 'static,
{
    fn pack(self) -> Box<Future<Item = I, Error = E> + Send> {
        Box::new(self)
    }
}

pub type FutResult<T> = Box<Future<Item = T, Error = Error> + Send>;

/// The `OauthClient` trait allows to generate the key components for
/// each of the [RFC 6749](https://tools.ietf.org/html/rfc6749) client side steps
pub trait OauthClient<S>: Sized
where
    S: ClientStorage<Self>,
{
    type Request;
    type Response;
    // TODO: Add Type Error
    /// Used to implement [4.1.1](https://tools.ietf.org/html/rfc6749#section-4.1.1) and
    /// [4.2.1](https://tools.ietf.org/html/rfc6749#section-4.2.1) Authorization Request
    fn get_user_auth_request(&self, storage: &mut S) -> FutResult<Self::Request>;

    /// Handles the [4.1.2](https://tools.ietf.org/html/rfc6749#section-4.1.2) Authorization Redirect Request
    fn handle_auth_request(request: Self::Request, storage: &mut S) -> FutResult<Self>;

    /// Used to implement [4.1.3](https://tools.ietf.org/html/rfc6749#section-4.1.3) Token Request
    fn get_access_token_request(&self) -> FutResult<Self::Request>;

    /// Handles the [4.1.4](https://tools.ietf.org/html/rfc6749#section-4.1.4) Token Response
    fn handle_token_response(self, response: Self::Response, storage: &mut S) -> FutResult<Self>;

    // Used to implement [4.6](https://tools.ietf.org/html/rfc6749#section-4.1.4) Token Refresh Reqeust
    // fn get_token_refresh_request(self, response: Self::Response) -> FutureResult<Self, Error>;
}

/// Used to Storage Client between the authentication Steps
pub trait ClientStorage<C: Sized + OauthClient<Self>>: Sized {
    type Error;
    type Lookup;

    fn set(&mut self, lookup: Self::Lookup, value: C) -> FutResult<Option<C>>;
    fn get(&self, lookup: Self::Lookup) -> FutResult<C>;
    fn drop(&mut self, lookup: Self::Lookup) -> FutResult<C>;
    fn has(&self, lookup: Self::Lookup) -> FutResult<bool>;
}

struct ValidReq {
    code: String,
    state: Option<String>,
}

impl ValidReq {
    fn from_url<T: Into<params::UrlQueryParams> + Clone>(into_params: &T) -> Result<ValidReq> {
        let params: params::UrlQueryParams = into_params.clone().into();
        let code: String = params
            .get("code")
            .ok_or("Requires a code to authorize token")?
            .single()
            .ok_or("Expected the code to be a single value")?
            .clone();
        let state = match params.get("state") {
            Some(n) => n.single().cloned(),
            None => None,
        };

        Ok(ValidReq { code, state })
    }
}

/// [4.2.2.  Access Token Response](https://tools.ietf.org/html/rfc6749#section-4.2.2)
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub token_type: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_in: Option<usize>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

impl TokenResponse {
    pub fn new<T: Into<String>, S: Into<String>>(access_token: T, token_type: S) -> TokenResponse {
        TokenResponse {
            access_token: access_token.into(),
            token_type: token_type.into(),
            expires_in: None,
            scope: None,
            state: None,
        }
    }

    pub fn with_state<T: Into<String>>(self, state: T) -> TokenResponse {
        TokenResponse {
            access_token: self.access_token,
            token_type: self.token_type,
            expires_in: self.expires_in,
            scope: self.scope,
            state: Some(state.into()),
        }
    }

    pub fn with_scope<T: Into<String>>(self, scope: Vec<T>) -> TokenResponse {
        TokenResponse {
            access_token: self.access_token,
            token_type: self.token_type,
            expires_in: self.expires_in,
            scope: Some(scope.into_iter().map(|v| v.into()).collect()),
            state: self.state,
        }
    }

    pub fn with_expiration(self, expiration: usize) -> TokenResponse {
        TokenResponse {
            access_token: self.access_token,
            token_type: self.token_type,
            expires_in: Some(expiration),
            scope: self.scope,
            state: self.state,
        }
    }
}

/// [4.2.2.1.  Error Response](https://tools.ietf.org/html/rfc6749#section-4.2.2.1)
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: &'static str,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_uri: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

impl ErrorResponse {
    #[inline]
    pub fn with_state(self, state: String) -> ErrorResponse {
        ErrorResponse {
            error: self.error,
            error_description: self.error_description,
            error_uri: self.error_uri,
            state: Some(state),
        }
    }
}

impl<'a> From<&'a ErrorKind> for ErrorResponse {
    #[inline]
    fn from(kind: &'a ErrorKind) -> ErrorResponse {
        let (error, error_description, error_uri): (
            &'static str,
            Option<String>,
            Option<String>,
        ) = match kind {
            ErrorKind::Msg(msg) => ("server_error", Some(msg.clone()), None),
            ErrorKind::InvalidRequest(desc, uri) => ("invalid_request", desc.clone(), uri.clone()),
            ErrorKind::InvalidClient(desc, uri) => ("invalid_client", desc.clone(), uri.clone()),
            ErrorKind::InvalidGrant(desc, uri) => ("invalid_grant", desc.clone(), uri.clone()),
            ErrorKind::UnauthorizedClient(desc, uri) => {
                ("unauthorized_client", desc.clone(), uri.clone())
            }
            ErrorKind::UnsupportedGrantType(desc, uri) => {
                ("unsupported_grant_type", desc.clone(), uri.clone())
            }
            ErrorKind::InvalidScope(desc, uri) => ("invalid_scope", desc.clone(), uri.clone()),
            _ => (
                "unknown_error",
                Some("Failed to Recongize Given ErrorKind".to_string()),
                None,
            ),
        };
        ErrorResponse {
            error,
            error_description: error_description,
            error_uri: error_uri,
            state: None,
        }
    }
}

impl<'a> From<&'a Error> for ErrorResponse {
    #[inline]
    fn from(e: &'a Error) -> ErrorResponse {
        e.kind().into()
    }
}

impl IntoIterator for ErrorResponse {
    type Item = (&'static str, String);
    type IntoIter = ::std::vec::IntoIter<(&'static str, String)>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        let mut out = vec![("error", self.error.into())];

        match self.error_description {
            Some(desc) => out.push(("error_description", desc)),
            None => (),
        };
        match self.error_uri {
            Some(uri) => out.push(("error_uri", uri)),
            None => (),
        };
        match self.state {
            Some(state) => out.push(("state", state)),
            None => (),
        };
        out.into_iter()
    }
}
