//! Client for RFC 6749, a.k.a, OAuth 2.0 Framework
pub mod authenticator;
pub mod mock_client;
pub mod params;
pub mod storage;

#[cfg(test)]
pub mod params_test;
#[cfg(test)]
pub mod storage_test;
#[cfg(test)]
pub mod mock_client_test;
#[cfg(test)]
pub mod authenticator_test;

use futures::future::{Future, FutureResult};
use errors::Error;

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
    fn get_user_auth_request(
        &self,
        storage: &mut S,
    ) -> Box<Future<Item = Self::Request, Error = Error> + Send>;

    /// Handles the [4.1.2](https://tools.ietf.org/html/rfc6749#section-4.1.2) Authorization Redirect Request
    fn handle_auth_request(request: Self::Request, storage: &mut S) -> FutureResult<Self, Error>;

    /// Used to implement [4.1.3](https://tools.ietf.org/html/rfc6749#section-4.1.3) Token Request
    fn get_user_token_request(&self, storage: &mut S) -> FutureResult<Self::Response, Error>;

    /// Handles the [4.1.4](https://tools.ietf.org/html/rfc6749#section-4.1.4) Token Response
    fn handle_token_response(
        self,
        response: Self::Response,
        storage: &mut S,
    ) -> FutureResult<Self, Error>;

    // Used to implement [4.6](https://tools.ietf.org/html/rfc6749#section-4.1.4) Token Refresh Reqeust
    // fn get_token_refresh_request(self, response: Self::Response) -> FutureResult<Self, Error>;
}

/// Used to Storage Client between the authentication Steps
pub trait ClientStorage<C: Sized + OauthClient<Self>>: Sized {
    type Error;
    type Lookup;

    fn set(&mut self, lookup: Self::Lookup, value: C) -> FutureResult<Option<C>, Self::Error>;
    fn get(&self, lookup: Self::Lookup) -> FutureResult<C, Self::Error>;
    fn drop(&mut self, lookup: Self::Lookup) -> FutureResult<C, Self::Error>;
    fn has(&self, lookup: Self::Lookup) -> FutureResult<bool, Self::Error>;
}
