//! Client for RFC 6749, a.k.a, OAuth 2.0 Framework
pub mod authenticator;
pub mod simple_client;

use futures::future::FutureResult;
use errors::Error;

/// The `OauthClient` trait allows to generate the key components for
/// each of the [RFC 6749](https://tools.ietf.org/html/rfc6749) client side steps
pub trait OauthClient: Sized {
    type Request;
    type Response;
    /// Used to implement [4.1.1](https://tools.ietf.org/html/rfc6749#section-4.1.1) and
    /// [4.2.1](https://tools.ietf.org/html/rfc6749#section-4.2.1) Authorization Request
    fn get_user_auth_request(&self) -> FutureResult<Self::Request, Error>;

    /// Handles the [4.1.2](https://tools.ietf.org/html/rfc6749#section-4.1.2) Authorization Redirect Request
    fn handle_auth_request(self, request: Self::Request) -> FutureResult<Self, Error>;

    /// Used to implement [4.1.3](https://tools.ietf.org/html/rfc6749#section-4.1.3) Token Request
    fn get_user_token_request(&self) -> FutureResult<Self::Response, Error>;

    /// Handles the [4.1.4](https://tools.ietf.org/html/rfc6749#section-4.1.4) Token Response
    fn handle_token_response(self, response: Self::Response) -> FutureResult<Self, Error>;

    /// Used to implement [4.6](https://tools.ietf.org/html/rfc6749#section-4.1.4) Token Refresh Reqeust
    fn get_token_refresh_request(self, response: Self::Response) -> FutureResult<Self, Error>;
}
