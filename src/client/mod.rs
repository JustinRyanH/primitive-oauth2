//! Client for RFC 6749, a.k.a, OAuth 2.0 Framework
pub mod authenticator;
pub mod mock_client;
pub mod mock_server;
pub mod params;
pub mod requests;
pub mod responses;
pub mod storage;
pub mod token;

#[cfg(test)]
pub mod mock_client_spec;

pub use self::requests::MockReq;
pub use self::responses::{ErrorResponse, MockResp, TokenResponse};

use serde::{
    de::{Error as DeError, Unexpected}, Deserialize, Deserializer, Serialize, Serializer,
};

use client::params::ParamValue;
use errors::{ErrorKind, OAuthResult};
use futures::future::Future;

pub struct Token {
    access_type: String,
    token_type: String,
    expires_in: Option<usize>,
    refresh_token: Option<String>,
    scope: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub enum AccessType {
    Implicit,
    Grant,
}

// This can totally be a Trait and we can make AccessType
// automagical
impl AccessType {
    pub fn get_response_type(&self) -> &'static str {
        match self {
            &AccessType::Implicit => "token",
            &AccessType::Grant => "code",
        }
    }

    pub fn get_grant_type(&self) -> &'static str {
        match self {
            &AccessType::Implicit => "refresh_token",
            &AccessType::Grant => "authorization_code",
        }
    }

    pub fn from_param_value<'a>(param: &ParamValue<'a>) -> OAuthResult<AccessType> {
        let result = param.single().ok_or(ErrorKind::unsupported_grant_type(
            Some("`grant_type` requires a single string"),
            None,
        ))?;
        match result.as_ref() {
            "authorization_code" => Ok(AccessType::Grant),
            _ => Err(ErrorKind::invalid_grant(
                Some(format!("`{:?}` is not a valid grant type", param)),
                None,
            )),
        }
    }
}

impl Serialize for AccessType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(match *self {
            AccessType::Implicit => "implicit",
            AccessType::Grant => "grant",
        })
    }
}

impl<'de> Deserialize<'de> for AccessType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if s.as_str().to_lowercase() == "implicit" {
            Ok(AccessType::Implicit)
        } else if s.as_str().to_lowercase() == "grant" {
            Ok(AccessType::Grant)
        } else {
            Err(DeError::invalid_value(
                Unexpected::Str(&s.as_str().to_lowercase()),
                &"Grant or Implicit",
            ))
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
    E: Into<ErrorKind> + 'static,
{
    fn pack(self) -> Box<Future<Item = I, Error = E> + Send> {
        Box::new(self)
    }
}

/// The `OauthClient` trait allows to generate the key components for
/// each of the [RFC 6749](https://tools.ietf.org/html/rfc6749) client side steps
pub trait OauthClient: Sized {
    type Request;
    type Response;
    // TODO: Add Type Error
    /// Used to implement [4.1.1](https://tools.ietf.org/html/rfc6749#section-4.1.1) and
    /// [4.2.1](https://tools.ietf.org/html/rfc6749#section-4.2.1) Authorization Request
    fn get_user_auth_request<'a, 'b, S>(&'b self, storage: &'a mut S) -> OAuthResult<Self::Request>
    where
        S: ClientStorage<'a, Self>;

    /// Handles the [4.1.2](https://tools.ietf.org/html/rfc6749#section-4.1.2) Authorization Redirect Request
    fn handle_auth_redirect<'a, S>(
        state_required: bool,
        request: Self::Request,
        storage: &'a mut S,
    ) -> OAuthResult<Self>
    where
        S: ClientStorage<'a, Self>;

    /// Used to implement [4.1.3](https://tools.ietf.org/html/rfc6749#section-4.1.3) Token Request
    fn get_access_token_request(&self) -> OAuthResult<Self::Request>;

    /// Handles the [4.1.4](https://tools.ietf.org/html/rfc6749#section-4.1.4) Token Response
    fn handle_token_response<'a, S>(
        self,
        response: Self::Response,
        storage: &'a mut S,
    ) -> OAuthResult<Self>
    where
        S: ClientStorage<'a, Self>;

    // Used to implement [4.6](https://tools.ietf.org/html/rfc6749#section-4.1.4) Token Refresh Reqeust
    // fn get_token_refresh_request(self, response: Self::Response) -> FutureResult<Self, Error>;
}

/// Used to Storage Client between the authentication Steps
pub trait ClientStorage<'a, C: Sized + OauthClient>: Sized {
    type Error;

    fn set<K>(&mut self, lookup: K, value: C) -> OAuthResult<Option<C>>
    where
        K: Into<String>;
    fn get<K>(&self, lookup: K) -> OAuthResult<C>
    where
        K: Into<String>;
    fn drop<K>(&mut self, lookup: K) -> OAuthResult<C>
    where
        K: Into<String>;
    fn has<K>(&self, lookup: K) -> OAuthResult<bool>
    where
        K: Into<String>;
}
