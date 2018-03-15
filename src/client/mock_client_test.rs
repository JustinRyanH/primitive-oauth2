use std::iter::FromIterator;

use futures::Future;
use futures_cpupool::CpuPool;
use rspec::{self, given};
use spectral::prelude::*;

use client::mock_client::*;
use client::params::{ParamValue, UrlQueryParams};
use client::OauthClient;

#[test]
fn mock_client() {
    rspec::run(&given("A Mock Client", CpuPool::new(4), |ctx| {
        ctx.context("Generating Auth Request for User", |ctx| {
            ctx.it(
                "Generates a request with the authenticator `client_id`",
                |pool| {
                    let result_params = pool.clone()
                        .spawn(
                            MockClient::new()
                                .unwrap()
                                .get_user_auth_request()
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
                |pool| {
                    let result_params = pool.spawn(
                        MockClient::new()
                            .unwrap()
                            .get_user_auth_request()
                            .and_then(|req| Ok(UrlQueryParams::from(req.url))),
                    ).wait()
                        .unwrap();
                    // Params from [RFC](https://tools.ietf.org/html/rfc6749#section-4.1.1)
                    assert_that(&*result_params)
                        .contains_key("redirect_uri".to_string())
                        .is_equal_to(ParamValue::from("https://localhost/auth"));
                },
            );

            ctx.it("Generates a request with the example `scope`", |pool| {
                let result_params = pool.spawn(
                    MockClient::new()
                        .unwrap()
                        .get_user_auth_request()
                        .and_then(|req| Ok(UrlQueryParams::from(req.url))),
                ).wait()
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
                "Generates a request with the example `request_type`",
                |pool| {
                    let result_params = pool.spawn(
                        MockClient::new()
                            .unwrap()
                            .get_user_auth_request()
                            .and_then(|req| Ok(UrlQueryParams::from(req.url))),
                    ).wait()
                        .unwrap();
                    // Params from [RFC](https://tools.ietf.org/html/rfc6749#section-4.1.1)
                    assert_that(&*result_params)
                        .contains_key("response_type".to_string())
                        .is_equal_to(ParamValue::from("code"));
                },
            );
        });

        ctx.context("Handles auth request from auth server", |_| {})
    }));
}
