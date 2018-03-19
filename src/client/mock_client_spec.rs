use std::iter::FromIterator;
use std::sync::Arc;

use futures::Future;
use futures::future::{err as FutErr, ok as FutOk};
use futures_cpupool::CpuPool;
use rspec::{self, given};
use spectral::prelude::*;

use url::Url;

use client::{AsyncPacker, FutResult, OauthClient};
use client::storage::MockMemoryStorage;
use client::params::{ParamValue, UrlQueryParams};
use client::mock_client::*;
use errors::Error;

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

#[test]
fn mock_client() {
    #[derive(Debug, Clone)]
    struct Env {
        pool: CpuPool,
        storage: Arc<MockMemoryStorage>,
    };

    impl Default for Env {
        fn default() -> Env {
            Env {
                pool: CpuPool::new(4),
                storage: Arc::new(MockMemoryStorage::new()),
            }
        }
    }
    rspec::run(&given("A Mock Client", Env::default(), |ctx| {
        ctx.context("Generating Auth Request for User", |ctx| {
            ctx.it(
                "Generates a request with the authenticator `client_id`",
                |env| {
                    let result_params = env.pool
                        .spawn(
                            MockClient::new()
                                .unwrap()
                                .get_user_auth_request(Arc::make_mut(&mut env.storage.clone()))
                                .and_then(|req| Ok(UrlQueryParams::from(req.url))),
                        )
                        .wait()
                        .unwrap();

                    // Params from [RFC](https://tools.ietf.org/html/rfc6749#section-4.1.1)
                    assert_that(&*result_params)
                        .contains_key("client_id".to_string())
                        .is_equal_to(ParamValue::from("someid@example.com"));
                },
            );

            ctx.it(
                "Generates a request with the example `redirect_uri`",
                |env| {
                    let result_params = env.pool
                        .spawn(
                            MockClient::new()
                                .unwrap()
                                .get_user_auth_request(Arc::make_mut(&mut env.storage.clone()))
                                .and_then(|req| Ok(UrlQueryParams::from(req.url))),
                        )
                        .wait()
                        .unwrap();

                    // Params from [RFC](https://tools.ietf.org/html/rfc6749#section-4.1.1)
                    assert_that(&*result_params)
                        .contains_key("redirect_uri".to_string())
                        .is_equal_to(ParamValue::from("https://localhost/auth"));
                },
            );

            ctx.it("Generates a request with the example `scope`", |env| {
                let result_params = env.pool
                    .spawn(
                        MockClient::new()
                            .unwrap()
                            .get_user_auth_request(Arc::make_mut(&mut env.storage.clone()))
                            .and_then(|req| Ok(UrlQueryParams::from(req.url))),
                    )
                    .wait()
                    .unwrap();

                // Params from [RFC](https://tools.ietf.org/html/rfc6749#section-4.1.1)
                assert_that(&*result_params)
                    .contains_key("scope".to_string())
                    .is_equal_to(ParamValue::from_iter(vec![
                        "api.example.com/user.profile",
                        "api.example.com/user.me",
                    ]));
            });

            ctx.it(
                "Generates a request with the example `response_type`",
                |env| {
                    let result_params = env.pool
                        .spawn(
                            MockClient::new()
                                .unwrap()
                                .get_user_auth_request(Arc::make_mut(&mut env.storage.clone()))
                                .and_then(|req| Ok(UrlQueryParams::from(req.url))),
                        )
                        .wait()
                        .unwrap();

                    // Params from [RFC](https://tools.ietf.org/html/rfc6749#section-4.1.1)
                    assert_that(&*result_params)
                        .contains_key("response_type".to_string())
                        .is_equal_to(ParamValue::from("code"));
                },
            );

            ctx.it("Generates a request with the example `state`", |env| {
                let result_params = env.pool
                    .spawn(
                        MockClient::new()
                            .unwrap()
                            .get_user_auth_request(Arc::make_mut(&mut env.storage.clone()))
                            .and_then(|req| Ok(UrlQueryParams::from(req.url))),
                    )
                    .wait()
                    .unwrap();

                // Params from [RFC](https://tools.ietf.org/html/rfc6749#section-4.1.1)
                assert_that(&*result_params)
                    .contains_key("state".to_string())
                    .is_equal_to(ParamValue::from("EXAMPLE_STATE"));
            });
        });

        ctx.context("Handles auth request from auth server", |ctx| {
            ctx.it(
                "then makes a mock client that can handle token requests",
                move |env| {
                    let storage = env.storage.clone();
                    let auth_request = env.pool
                        .spawn(
                            MockClient::new()
                                .unwrap()
                                .get_user_auth_request(Arc::make_mut(&mut storage.clone()))
                                .and_then(|req| MockServer::new().redirect(req))
                                .and_then(move |req| {
                                    MockClient::handle_auth_request(
                                        req,
                                        Arc::make_mut(&mut storage.clone()),
                                    )
                                }),
                        )
                        .wait();
                    assert_that(&auth_request)
                        .is_ok()
                        .is_equal_to(MockClient::new().unwrap().with_code("MOCK_CODE"));
                },
            )
        })
    }));
}
