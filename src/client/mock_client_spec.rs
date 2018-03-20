use std::iter::FromIterator;
use std::sync::Arc;

use futures::Future;
use futures::future::{err as FutErr, ok as FutOk};
use futures_cpupool::CpuPool;
use rspec::{self, given};

use url::Url;

use client::{AsyncPacker, FutResult};
use client::storage::MockMemoryStorage;
use client::params::{ParamValue, UrlQueryParams};
use client::mock_client::*;
use errors::{Error, Result};

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
pub struct MockError {
    pub kind: MockErrKind,
    pub description: Option<String>,
    pub url: Option<String>,
}

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
}

mod given_mock_client {
    use client::OauthClient;
    use spectral::prelude::*;
    use super::*;

    #[derive(Debug, Clone)]
    struct Env {
        pool: CpuPool,
        storage: MockMemoryStorage,
    }

    fn env() -> Env {
        Env {
            pool: CpuPool::new(1),
            storage: MockMemoryStorage::new(),
        }
    }

    mod get_user_auth_request {
        use super::*;

        fn subject() -> UrlQueryParams {
            let env = env();
            let subject_of_interest = MockClient::new()
                .unwrap()
                .get_user_auth_request(&mut env.storage.clone());

            env.pool
                .clone()
                .spawn(subject_of_interest)
                .wait()
                .unwrap()
                .into()
        }

        #[test]
        fn request_contains_client_id() {
            assert_that(&*subject())
                .contains_key("client_id".to_string())
                .is_equal_to(ParamValue::from("someid@example.com"));
        }

        #[test]
        fn request_contains_redirect_uri() {
            assert_that(&*subject())
                .contains_key("redirect_uri".to_string())
                .is_equal_to(ParamValue::from("https://localhost/auth"));
        }

        #[test]
        fn request_contains_scope() {
            assert_that(&*subject())
                .contains_key("scope".to_string())
                .is_equal_to(ParamValue::from_iter(vec![
                    "api.example.com/user.profile",
                    "api.example.com/user.me",
                ]));
        }

        #[test]
        fn request_contains_response_type() {
            assert_that(&*subject())
                .contains_key("response_type".to_string())
                .is_equal_to(ParamValue::from("code"));
        }

        #[test]
        fn request_contains_state() {
            assert_that(&*subject())
                .contains_key("state".to_string())
                .is_equal_to(ParamValue::from("EXAMPLE_STATE"));
        }
    }

    mod handle_auth_request {
        use client::OauthClient;
        use super::*;

        fn subject(server: MockServer) -> Result<MockClient> {
            let env = env();
            let storage = env.storage.clone();
            let subject_of_interest = MockClient::new()
                .unwrap()
                .get_user_auth_request(&mut storage.clone())
                .and_then(move |req| server.redirect(req))
                .and_then(move |req| MockClient::handle_auth_request(req, &mut storage.clone()));

            env.clone().pool.clone().spawn(subject_of_interest).wait()
        }

        mod when_there_is_a_previous_state {
            use super::*;
            fn server() -> MockServer {
                MockServer::new()
            }

            #[test]
            fn it_creates_a_client_from_state_and_code() {
                assert_that(&subject(server()))
                    .is_ok()
                    .is_equal_to(MockClient::new().unwrap().with_code("MOCK_CODE"));
            }
        }
    }
}
