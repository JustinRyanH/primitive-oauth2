//! Client for RFC 6749, a.k.a, OAuth 2.0 Framework
pub mod authenticator;
pub mod mock_client;
pub mod mock_server;
pub mod params;
pub mod requests;
pub mod responses;
pub mod storage;

#[cfg(test)]
pub mod mock_client_spec;

pub use self::requests::MockReq;
pub use self::responses::{ErrorResponse, MockResp, TokenResponse};

use serde::{de::{Error as DeError, Unexpected},
            Deserialize,
            Deserializer,
            Serialize,
            Serializer};

use errors::{ErrorKind, OAuthResult};
use futures::future::Future;

#[derive(Debug, Clone, Copy, PartialEq, Hash)]
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
pub trait OauthClient<S>: Sized
where
    S: ClientStorage<Self>,
{
    type Request;
    type Response;
    // TODO: Add Type Error
    /// Used to implement [4.1.1](https://tools.ietf.org/html/rfc6749#section-4.1.1) and
    /// [4.2.1](https://tools.ietf.org/html/rfc6749#section-4.2.1) Authorization Request
    fn get_user_auth_request(&self, storage: &mut S) -> OAuthResult<Self::Request>;

    /// Handles the [4.1.2](https://tools.ietf.org/html/rfc6749#section-4.1.2) Authorization Redirect Request
    fn handle_auth_redirect(request: Self::Request, storage: &mut S) -> OAuthResult<Self>;

    /// Used to implement [4.1.3](https://tools.ietf.org/html/rfc6749#section-4.1.3) Token Request
    fn get_access_token_request(&self) -> OAuthResult<Self::Request>;

    /// Handles the [4.1.4](https://tools.ietf.org/html/rfc6749#section-4.1.4) Token Response
    fn handle_token_response(self, response: Self::Response, storage: &mut S) -> OAuthResult<Self>;

    // Used to implement [4.6](https://tools.ietf.org/html/rfc6749#section-4.1.4) Token Refresh Reqeust
    // fn get_token_refresh_request(self, response: Self::Response) -> FutureResult<Self, Error>;
}

/// Used to Storage Client between the authentication Steps
pub trait ClientStorage<C: Sized + OauthClient<Self>>: Sized {
    type Error;
    type Lookup;

    fn set(&mut self, lookup: Self::Lookup, value: C) -> OAuthResult<Option<C>>;
    fn get(&self, lookup: Self::Lookup) -> OAuthResult<C>;
    fn drop(&mut self, lookup: Self::Lookup) -> OAuthResult<C>;
    fn has(&self, lookup: Self::Lookup) -> OAuthResult<bool>;
}
