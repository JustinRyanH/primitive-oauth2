use futures::future::{result, FutureResult};
use url::Url;
use serde::de::DeserializeOwned;

use errors::{Error, Result};
use client::OauthClient;
use client::authenticator::BaseAuthenticator;

#[derive(Debug, Clone, Copy)]
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

pub struct MockReq<T>
where
    T: DeserializeOwned,
{
    pub url: Url,
    pub body: T,
}

pub struct MockResp<T>
where
    T: DeserializeOwned,
{
    pub body: T,
}

#[derive(Debug, Clone)]
pub struct MockClient {
    pub auth: BaseAuthenticator,
    pub scopes: Vec<String>,
    pub redirect_uri: &'static str,
    pub access_type: AccessType,
}

impl MockClient {
    pub fn new() -> Result<MockClient> {
        Ok(MockClient {
            auth: BaseAuthenticator::new(
                "someid@example.com",
                "test",
                Url::parse("http://example.com/auth")?,
                Url::parse("http://example.com/token")?,
            ),
            scopes: vec![
                "api.example.com/user.profile".to_string(),
                "api.example.com/user.me".to_string(),
            ],
            redirect_uri: "https://localhost/auth",
            access_type: AccessType::Grant,
        })
    }
}

impl OauthClient for MockClient {
    type Request = MockReq<String>;
    type Response = MockResp<String>;

    fn get_user_auth_request(&self) -> FutureResult<MockReq<String>, Error> {
        let url = match Url::parse_with_params(
            self.auth.get_auth_uri(),
            self.auth.get_auth_params(&self.redirect_uri, &self.scopes),
        ) {
            Ok(u) => u,
            Err(e) => return result(Err(e.into())),
        };
        result(Ok(MockReq {
            url: url,
            body: String::from(""),
        }))
    }

    fn handle_auth_request(_: MockReq<String>) -> FutureResult<Self, Error> {
        unimplemented!()
    }

    fn get_user_token_request(&self) -> FutureResult<MockResp<String>, Error> {
        unimplemented!()
    }

    fn handle_token_response(self, _: MockResp<String>) -> FutureResult<Self, Error> {
        unimplemented!()
    }
}
