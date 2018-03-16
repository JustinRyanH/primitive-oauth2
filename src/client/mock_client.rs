use futures::future::{Future, FutureResult};
use url::Url;
use serde::de::DeserializeOwned;

use errors::{Error, Result};
use client::{AccessType, AsyncPacker, ClientStorage, FutResult, OauthClient};
use client::storage::{MockMemoryStorage, MockStorageKey};
use client::authenticator::BaseAuthenticator;

pub struct MockReq<T>
where
    T: DeserializeOwned + Sized,
{
    pub url: Url,
    pub body: T,
}

pub struct MockResp<T>
where
    T: DeserializeOwned + Sized,
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

    fn get_user_auth_request(&self, storage: &mut MockMemoryStorage) -> FutResult<MockReq<String>> {
        let state = "EXAMPLE_STATE";
        let mock_req = match Url::parse_with_params(
            self.auth.get_auth_uri(),
            self.auth
                .get_auth_params(&self.redirect_uri, &self.scopes, self.access_type, state),
        ) {
            Ok(url) => Ok(MockReq {
                url,
                body: String::from(""),
            }),
            Err(e) => Err(e.into()),
        };

        storage
            .set(MockStorageKey::state(state), self.clone())
            .and_then(move |_| mock_req)
            .pack()
    }

    fn handle_auth_request(_: MockReq<String>, _: &mut MockMemoryStorage) -> FutResult<Self> {
        unimplemented!()
    }

    fn get_user_token_request(&self, _: &mut MockMemoryStorage) -> FutResult<MockResp<String>> {
        unimplemented!()
    }

    fn handle_token_response(
        self,
        _: MockResp<String>,
        _: &mut MockMemoryStorage,
    ) -> FutResult<Self> {
        unimplemented!()
    }
}
