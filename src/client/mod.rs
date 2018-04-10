//! Client for RFC 6749, a.k.a, OAuth 2.0 Framework
pub mod authenticator;
pub mod mock_client;
pub mod mock_server;
pub mod params;
pub mod responses;
pub mod storage;

#[cfg(test)]
pub mod authenticator_spec;
// #[cfg(test)]
// pub mod mock_client_spec;

pub use self::responses::{ErrorResponse, TokenResponse};

use errors::{Error, Result};
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
    fn get_user_auth_request(&self, storage: &mut S) -> Result<Self::Request>;

    /// Handles the [4.1.2](https://tools.ietf.org/html/rfc6749#section-4.1.2) Authorization Redirect Request
    fn handle_auth_redirect(request: Self::Request, storage: &mut S) -> Result<Self>;

    /// Used to implement [4.1.3](https://tools.ietf.org/html/rfc6749#section-4.1.3) Token Request
    fn get_access_token_request(&self) -> Result<Self::Request>;

    /// Handles the [4.1.4](https://tools.ietf.org/html/rfc6749#section-4.1.4) Token Response
    fn handle_token_response(self, response: Self::Response, storage: &mut S) -> Result<Self>;

    // Used to implement [4.6](https://tools.ietf.org/html/rfc6749#section-4.1.4) Token Refresh Reqeust
    // fn get_token_refresh_request(self, response: Self::Response) -> FutureResult<Self, Error>;
}

/// Used to Storage Client between the authentication Steps
pub trait ClientStorage<C: Sized + OauthClient<Self>>: Sized {
    type Error;
    type Lookup;

    fn set(&mut self, lookup: Self::Lookup, value: C) -> Result<Option<C>>;
    fn get(&self, lookup: Self::Lookup) -> Result<C>;
    fn drop(&mut self, lookup: Self::Lookup) -> Result<C>;
    fn has(&self, lookup: Self::Lookup) -> Result<bool>;
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
