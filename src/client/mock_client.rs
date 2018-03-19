use futures::future::{err as FutErr, ok as FutOk};
use futures::future::{Future, IntoFuture};
use url::Url;

use errors::{Error, Result};
use client::{AccessType, AsyncPacker, ClientStorage, FutResult, OauthClient, ValidReq};
use client::storage::{MockMemoryStorage, MockStorageKey};
use client::authenticator::BaseAuthenticator;
use client::params::{ParamValue, UrlQueryParams};

pub struct MockReq {
    pub url: Url,
    pub body: String,
}

pub struct MockResp {
    pub body: String,
}

pub struct MockServer;

impl MockServer {
    pub fn redirect(req: MockReq) -> FutResult<MockReq> {
        match req.url.path() {
            "/auth" => {
                let state = match UrlQueryParams::from(req.url.query_pairs())
                    .get("state")
                    .unwrap_or(ParamValue::from(""))
                    .single()
                {
                    Some(v) => v.clone(),
                    None => String::from(""),
                };
                FutOk(MockReq {
                    url: Url::parse_with_params(
                        "https://localhost/example/auth",
                        vec![("state", state), ("code", "MOCK_CODE".into())],
                    ).unwrap(),
                    body: String::from(""),
                }).pack()
            }
            _ => FutErr(Error::msg("404 Route not found")).pack(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MockClient {
    pub auth: BaseAuthenticator,
    pub scopes: Vec<String>,
    pub redirect_uri: &'static str,
    pub access_type: AccessType,
    pub code: Option<String>,
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
            code: None,
        })
    }

    pub fn with_code<S: Into<String>>(self, code: S) -> MockClient {
        MockClient {
            auth: self.auth,
            scopes: self.scopes,
            redirect_uri: self.redirect_uri,
            access_type: self.access_type,
            code: Some(code.into()),
        }
    }
}

impl OauthClient<MockMemoryStorage> for MockClient {
    type Request = MockReq;
    type Response = MockResp;

    fn get_user_auth_request(&self, storage: &mut MockMemoryStorage) -> FutResult<MockReq> {
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

    fn handle_auth_request(req: MockReq, storage: &mut MockMemoryStorage) -> FutResult<Self> {
        let data = match ValidReq::from_url(&req.url) {
            Ok(d) => d,
            Err(e) => return FutErr(e.into()).pack(),
        };
        let code = data.code.clone();

        match data.state {
            Some(state) => storage
                .get(MockStorageKey::State(state))
                .and_then(|c| Ok(c.with_code(code)))
                .pack(),
            None => MockClient::new()
                .into_future()
                .and_then(|c| Ok(c.with_code(code)))
                .pack(),
        }
    }

    fn get_user_token_request(&self, _: &mut MockMemoryStorage) -> FutResult<MockResp> {
        unimplemented!()
    }

    fn handle_token_response(self, _: MockResp, _: &mut MockMemoryStorage) -> FutResult<Self> {
        unimplemented!()
    }
}
