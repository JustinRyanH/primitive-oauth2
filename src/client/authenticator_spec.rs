use std::iter::FromIterator;

use spectral::prelude::*;
use rspec::{self, given};
use dotenv;
use envy;

use client::params::{ParamValue, UrlQueryParams};
use client::AccessType;
use client::authenticator::*;

#[test]
fn authenticator_is_serializable() {
    dotenv::dotenv().expect("Failed to read the `.env` file");
    rspec::run(&given(
        "An BaseAuthenticator",
        BaseAuthenticator::default(),
        |ctx| {
            ctx.context(
                "When creating BaseAuthenticator with envy and/or another serde serializer",
                |ctx| {
                    ctx.before_each(|env| {
                        *env = envy::prefixed("EXAMPLE_OAUTH2_")
                            .from_env::<BaseAuthenticator>()
                            .ok()
                            .expect("Failed to Serialize BaseAuthenticator from .env");
                    });

                    ctx.context("#get_auth_params", |ctx| {
                        ctx.it("pushes the client_id in the params", |env| {
                            let result = UrlQueryParams::from_iter(env.get_auth_params(
                                "",
                                &vec![],
                                AccessType::Grant,
                                "STATE",
                            ));
                            assert_that(&*result)
                                .contains_key("client_id".to_string())
                                .is_equal_to(ParamValue::from(
                                    "example_foobar_whatever@example.com",
                                ));
                        });

                        ctx.it("pushes the redirect_uri into params", |env| {
                            let result = UrlQueryParams::from_iter(env.get_auth_params(
                                "https://localhost:8000",
                                &vec![],
                                AccessType::Grant,
                                "STATE",
                            ));
                            assert_that(&*result)
                                .contains_key("redirect_uri".to_string())
                                .is_equal_to(ParamValue::from("https://localhost:8000"));
                        });

                        ctx.it("pushes the scopes into params", |env| {
                            let result = UrlQueryParams::from_iter(env.get_auth_params(
                                "https:://localhost:8080",
                                &vec!["user.profile".to_string(), "user.openid".to_string()],
                                AccessType::Grant,
                                "STATE",
                            ));
                            assert_that(&*result)
                                .contains_key("scope".to_string())
                                .is_equal_to(ParamValue::from_iter(vec![
                                    "user.profile",
                                    "user.openid",
                                ]));
                        });
                    });

                    ctx.context("BaseAuthenticator Attributes", |ctx| {
                        ctx.it(
                            "then creates an BaseAuthenticator Object with a client id",
                            |env| {
                                let expected_client_id = "example_foobar_whatever@example.com";
                                let actual_client_id = env.get_client_id();
                                assert_that(&actual_client_id).is_equal_to(expected_client_id);
                            },
                        );

                        ctx.it(
                            "then creates an BaseAuthenticator Object with a client secret",
                            |env| {
                                let expected_client_secret = "super_secret";
                                let actual_client_secret = env.get_client_secret();
                                assert_that(&actual_client_secret)
                                    .is_equal_to(expected_client_secret);
                            },
                        );

                        ctx.it(
                            "then creates an BaseAuthenticator Object with a auth uri",
                            |env| {
                                let expected_auth_uri = "https://example.com/v1/auth";
                                let actual_auth_uri = env.get_auth_uri();
                                assert_that(&actual_auth_uri).is_equal_to(expected_auth_uri);
                            },
                        );

                        ctx.it(
                            "then creates an BaseAuthenticator Object with a token uri",
                            |env| {
                                let expected_token_uri = "https://example.com/v1/token";
                                let actual_token_uri = env.get_token_uri();
                                assert_that(&actual_token_uri).is_equal_to(expected_token_uri);
                            },
                        );
                    });
                },
            );
        },
    ));
}
