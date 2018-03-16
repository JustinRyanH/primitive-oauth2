use futures::future::{result, FutureResult, ok as FutOk};
use url::Url;
use serde::de::DeserializeOwned;

use errors::{Error, Result};
use client::{AccessType, OauthClient};
use client::storage::MockMemoryStorage;
use client::authenticator::BaseAuthenticator;

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

#[derive(Debug, Clone, PartialEq)]
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

impl OauthClient<MockMemoryStorage> for MockClient {
    type Request = MockReq<String>;
    type Response = MockResp<String>;

    fn get_user_auth_request(
        &self,
        _storage: &mut MockMemoryStorage,
    ) -> FutureResult<MockReq<String>, Error> {
        let url = match Url::parse_with_params(
            self.auth.get_auth_uri(),
            self.auth
                .get_auth_params(&self.redirect_uri, &self.scopes, self.access_type),
        ) {
            Ok(u) => u,
            Err(e) => return result(Err(e.into())),
        };
        FutOk(MockReq {
            url: url,
            body: String::from(""),
        })
    }

    fn handle_auth_request(
        _: MockReq<String>,
        _: &mut MockMemoryStorage,
    ) -> FutureResult<Self, Error> {
        unimplemented!()
    }

    fn get_user_token_request(
        &self,
        _: &mut MockMemoryStorage,
    ) -> FutureResult<MockResp<String>, Error> {
        unimplemented!()
    }

    fn handle_token_response(
        self,
        _: MockResp<String>,
        _: &mut MockMemoryStorage,
    ) -> FutureResult<Self, Error> {
        unimplemented!()
    }
}
