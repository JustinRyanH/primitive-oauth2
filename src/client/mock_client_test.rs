use std::iter::FromIterator;

use futures::Future;
use futures_cpupool::CpuPool;
use rspec::{self, given};
use spectral::prelude::*;

use client::mock_client::*;
use client::params::{ParamValue, UrlQueryParams};
use client::OauthClient;
use client::storage::MockMemoryStorage;

#[test]
fn mock_client() {
    #[derive(Debug, Clone)]
    struct Env {
        pool: CpuPool,
        storage: MockMemoryStorage,
    };

    impl Default for Env {
        fn default() -> Env {
            Env {
                pool: CpuPool::new(4),
                storage: MockMemoryStorage::new(),
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
                                .get_user_auth_request(&mut env.storage.clone())
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
                                .get_user_auth_request(&mut env.storage.clone())
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
                            .get_user_auth_request(&mut env.storage.clone())
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
                                .get_user_auth_request(&mut env.storage.clone())
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
                            .get_user_auth_request(&mut env.storage.clone())
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

        ctx.context("Handles auth request from auth server", |_| {})
    }));
}
