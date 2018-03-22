use futures::future::{err as FutErr, ok as FutOk};
use futures::future::{Future, IntoFuture};
use url::Url;

use errors::{Error, Result};
use client::params::{UrlQueryParams, ParamValue};
use client::OauthClient;
use client::{AccessType, AsyncPacker, ClientStorage, FutResult, ValidReq};
use client::storage::{MockMemoryStorage, MockStorageKey};
use client::authenticator::BaseAuthenticator;

#[derive(Debug, PartialEq, Clone)]
pub struct MockReq {
    pub url: Url,
    pub body: String,
}

impl Into<UrlQueryParams> for MockReq {
    fn into(self) -> UrlQueryParams {
        UrlQueryParams::from(self.url)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct MockResp {
    pub body: String,
}

impl<T> From<T> for MockResp where T: Into<String> {
    fn from(v: T) -> MockResp {
        MockResp {
            body: v.into(),
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
    pub server: MockServer,
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
            server: MockServer::new()
        })
    }

    pub fn with_code<S: Into<String>>(self, code: S) -> MockClient {
        MockClient {
            auth: self.auth,
            scopes: self.scopes,
            redirect_uri: self.redirect_uri,
            access_type: self.access_type,
            code: Some(code.into()),
            server: self.server,
        }
    }

    pub fn with_server(self, server: MockServer) -> MockClient {
        MockClient {
            auth: self.auth,
            scopes: self.scopes,
            redirect_uri: self.redirect_uri,
            access_type: self.access_type,
            code: self.code,
            server,
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

    fn request_token(&self) -> FutResult<MockResp> {
        let token_with_params: Url = match Url::parse_with_params(self.auth.get_token_uri(), vec![("foo", "bar")]) {
            Ok(k) => k,
            Err(e) => return FutErr(e.into()).pack(),
        };

        self.server.request(MockReq {
            url: token_with_params,
            body: String::from(""),
        })
    }

    fn handle_token_response(self, _: MockResp, _: &mut MockMemoryStorage) -> FutResult<Self> {
        unimplemented!()
    }
}


#[derive(Clone, Debug, PartialEq)]
pub enum MockErrKind {
    InvalidRequest,
    UnauthorizedClient,
    AccessDenied,
    UnsupportedResponseType,
    InvalidScope,
    ServerError,
    TemporarilyUnavailable,
    Unknown,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MockError {
    pub kind: MockErrKind,
    pub description: Option<String>,
    pub url: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MockServer {
    pub error: Option<MockError>,
}

impl MockServer {
    pub fn new() -> MockServer {
        MockServer { error: None }
    }

    pub fn with_error(error: MockError) -> MockServer {
        MockServer { error: Some(error) }
    }

    pub fn redirect(&self, req: MockReq) -> FutResult<MockReq> {
        match self.error {
            Some(ref e) => {
                let error_kind = e.kind.clone();
                match error_kind {
                    _ => unimplemented!(),
                }
            }
            None => match req.url.path() {
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
            },
        }
    }

    pub fn request(&self, req: MockReq) -> FutResult<MockResp> {
        match self.error {
            Some(_) => unimplemented!(),
            None => match req.url.path() {
                "/token" => FutOk(MockResp::from("{\"access_token\":\"2YotnFZFEjr1zCsicMWpAA\"}")).pack(),
                _ => FutErr(Error::msg("404 Route not found")).pack(),
            }
        }
    }
}

#[cfg(test)]
pub mod test_helpers {
    use super::*;
    use spectral::{AssertionFailure, Spec};

    pub trait MockClientHelper<'s> {
        fn has_code(&mut self) -> Spec<'s, String>;
        fn has_no_code(&mut self);
        fn has_access_type_of(&mut self, expected_type: AccessType);
        fn has_redirect_uri_of(&mut self, expected_uri: &'static str);
        fn has_scopes_of<'a, T: Clone + Into<String>>(&mut self, expected_scope: &'a Vec<T>);
    }

    impl<'s> MockClientHelper<'s> for Spec<'s, MockClient> {
        fn has_code(&mut self) -> Spec<'s, String> {
            match self.subject.code {
                Some(ref val) => Spec {
                    subject: val,
                    subject_name: self.subject_name,
                    location: self.location.clone(),
                    description: self.description,
                },
                None => {
                    AssertionFailure::from_spec(self)
                        .with_expected(format!("`MockClient.code` with Some(String)"))
                        .with_actual(format!("`MockClient.code` is None"))
                        .fail();
                    unreachable!();
                }
            }
        }

        fn has_no_code(&mut self) {
            match self.subject.code {
                None => (),
                Some(ref val) => {
                    AssertionFailure::from_spec(self)
                        .with_expected(format!("`MockClient.code` to be None"))
                        .with_actual(format!("`MockClient.code` is option<{:?}>", val))
                        .fail();
                    unreachable!();
                }
            }
        }

        fn has_access_type_of(&mut self, expected_type: AccessType) {
            let subject_type = self.subject.access_type;

            if subject_type == expected_type {
                ()
            } else {
                AssertionFailure::from_spec(self)
                    .with_expected(format!("`MockClient.access_type` of {:?}", expected_type))
                    .with_actual(format!("`MockClient.access_type` of {:?}", subject_type))
                    .fail();
                unreachable!();
            }
        }

        fn has_redirect_uri_of(&mut self, expected_uri: &'static str) {
            let subject_uri = self.subject.redirect_uri;
            if subject_uri == expected_uri {
                ()
            } else {
                AssertionFailure::from_spec(self)
                    .with_expected(format!("`MockClient.redirect_uri` of {:?}", expected_uri))
                    .with_actual(format!("`MockClient.redirect_uri` of {:?}", subject_uri))
                    .fail();
                unreachable!();
            }
        }

        fn has_scopes_of<'a, T: Clone + Into<String>>(&mut self, expected_scopes: &'a Vec<T>) {
            let subject_scopes = &self.subject.scopes;
            let ref parsed_scopes: Vec<String> = expected_scopes
                .into_iter()
                .map(|v| v.clone().into())
                .collect();

            if subject_scopes == parsed_scopes {
                ()
            } else {
                AssertionFailure::from_spec(self)
                    .with_expected(format!("`MockClient.scopes` of {:?}", parsed_scopes))
                    .with_actual(format!("`MockClient.scopes` of {:?}", subject_scopes))
                    .fail();
                unreachable!();
            }
        }
    }
}
