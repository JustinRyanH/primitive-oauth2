use std::borrow::Cow;

#[allow(unused_imports)]
use assertions::*;
use spectral::prelude::*;

use url::Url;

use client::authenticator::BaseAuthenticator;
use client::mock_client::MockClient;
use client::params::UrlQueryParams;
use client::requests::MockReq;
use client::responses::MockResp;
use client::storage::MemoryStorage;

#[inline]
fn storage() -> MemoryStorage {
    MemoryStorage::new()
}

#[inline]
fn base_auth() -> BaseAuthenticator {
    BaseAuthenticator::new(
        expected_client_id(),
        "http://example.com/auth",
        "http://example.com/token",
    ).unwrap()
}

#[inline]
fn expected_client_id() -> &'static str {
    "someid@example.com"
}

#[inline]
fn expected_client_secret() -> &'static str {
    "secret"
}

#[inline]
fn expected_redirect() -> &'static str {
    "https://localhost:8080/oauth/example"
}

#[inline]
fn expected_token_uri() -> &'static str {
    "http://example.com/token"
}

mod get_user_auth_request {
    use super::*;

    #[inline]
    fn mock_client() -> MockClient {
        MockClient::new(base_auth(), expected_redirect()).unwrap()
    }

    mod code_grant_flow {
        use super::*;

        mod request_host {
            use super::*;
            use client::OauthClient;

            #[test]
            fn it_sets_host_mock_clients_auth_uri() {
                let mut storage = storage();
                let req = mock_client().get_user_auth_request(&mut storage).unwrap();
                assert_that(&req.url.host_str())
                    .is_some()
                    .is_equal_to("example.com");
                assert_that(&req.url.scheme()).is_equal_to("http");
            }
        }

        mod response_type {
            use super::*;
            use client::OauthClient;

            #[test]
            fn it_sets_param_response_type_to_code() {
                let mut storage = storage();
                let req = mock_client().get_user_auth_request(&mut storage).unwrap();
                let params = UrlQueryParams::from(&req.url);
                assert_that(&params)
                    .has_param("response_type")
                    .have_a_single_value()
                    .is_equal_to(Cow::from("code"));
            }
        }
        mod client_id {
            use super::*;
            use client::OauthClient;

            #[test]
            fn it_sets_param_client_id_to_given_code_type() {
                let expected_client_id = base_auth().client_id;
                let mut storage = storage();
                let req = mock_client().get_user_auth_request(&mut storage).unwrap();
                let params = UrlQueryParams::from(&req.url);
                assert_that(&params)
                    .has_param("client_id")
                    .have_a_single_value()
                    .is_equal_to(Cow::from(expected_client_id));
            }
        }
        mod redirect_uri {
            use super::*;
            use client::OauthClient;

            #[test]
            fn it_sets_param_client_id_to_given_code_type() {
                let expected_redirect_uri = expected_redirect();
                let mut storage = storage();
                let req = mock_client().get_user_auth_request(&mut storage).unwrap();
                let params = UrlQueryParams::from(&req.url);
                assert_that(&params)
                    .has_param("redirect_uri")
                    .have_a_single_value()
                    .is_equal_to(Cow::from(expected_redirect_uri));
            }
        }

        mod scope {
            use super::*;

            mod when_there_is_no_scope {
                use super::*;
                use client::OauthClient;

                #[test]
                fn it_has_no_scope() {
                    assert_that(&mock_client().scope).has_length(0);
                }

                #[test]
                fn it_doesnt_supply_scope_in_params() {
                    let mut storage = storage();
                    let req = mock_client().get_user_auth_request(&mut storage).unwrap();
                    let params = UrlQueryParams::from(&req.url);
                    assert_that(&params).has_no_param("scope");
                }
            }

            // Foramts scope into space seperated list per https://tools.ietf.org/html/rfc6749#section-3.3
            mod when_there_is_scope {
                use super::*;
                use client::OauthClient;

                #[inline]
                fn mock_client() -> MockClient {
                    MockClient::new(base_auth(), expected_redirect())
                        .unwrap()
                        .with_scope(vec![
                            String::from("user.profile"),
                            String::from("account.profile"),
                        ])
                }

                #[test]
                fn it_has_some_scopes() {
                    assert_that(&mock_client().scope).contains::<String>("user.profile".into());
                    assert_that(&mock_client().scope).contains::<String>("account.profile".into());
                }

                #[test]
                fn it_supplies_scope_in_params() {
                    let mut storage = storage();
                    let req = mock_client().get_user_auth_request(&mut storage).unwrap();
                    let params = UrlQueryParams::from(&req.url);
                    assert_that(&params)
                        .has_param("scope")
                        .have_a_single_value()
                        .is_equal_to(Cow::from("user.profile account.profile"));
                }
            }
        }
        mod state {
            use super::*;
            mod when_state_is_on {
                use super::*;
                use client::OauthClient;

                #[inline]
                fn state() -> &'static str {
                    "FOOBAR_STATE"
                }

                #[inline]
                fn mock_client() -> MockClient {
                    MockClient::new(base_auth(), expected_redirect())
                        .unwrap()
                        .with_state(state())
                }

                #[test]
                fn assert_state_is_on() {
                    assert_that(&mock_client().state).is_some();
                }

                #[test]
                fn it_supplies_state_in_params() {
                    let mut storage = storage();
                    let req = mock_client().get_user_auth_request(&mut storage).unwrap();
                    let params = UrlQueryParams::from(&req.url);
                    assert_that(&params).has_param("state");
                }
            }
            mod when_state_is_off {
                use client::OauthClient;

                use super::*;

                #[test]
                fn assert_state_is_off() {
                    assert_that(&mock_client().state).is_none();
                }

                #[test]
                fn it_supplies_state_in_params() {
                    let mut storage = storage();
                    let req = mock_client().get_user_auth_request(&mut storage).unwrap();
                    let params = UrlQueryParams::from(&req.url);
                    assert_that(&params).has_no_param("state");
                }

                #[test]
                fn it_should_not_persist_the_client() {
                    let mut storage = storage();
                    assert_that(&*storage.read().unwrap()).is_empty();
                    let req = mock_client().get_user_auth_request(&mut storage);
                    assert_that(&req).is_ok();
                }
            }
        }
    }
    mod implicit_flow {
        // TODO: Implicit Flow
    }
    mod password_flow {
        // TODO: Password Flow
    }
    mod two_legged_auth {
        // TODO: Two-legged OAuth
    }
}

mod handle_auth_redirect {
    use super::*;

    mod when_happy {
        use super::*;

        mod when_there_is_state {
            use super::*;

            mod when_state_in_storage {
                use super::*;
                use client::ClientStorage;
                use client::OauthClient;

                /// Assertions
                fn mock_client() -> MockClient {
                    MockClient::new(base_auth(), expected_redirect())
                        .unwrap()
                        .with_state("MOCK_STATE")
                }

                #[inline]
                fn storage() -> MemoryStorage {
                    let mut storage = MemoryStorage::new();
                    storage.set("MOCK_STATE", mock_client()).unwrap();
                    return storage;
                }

                fn params() -> Vec<(&'static str, &'static str)> {
                    vec![
                        ("code", "MOCK_CODE"),
                        ("state", "MOCK_STATE"),
                        ("grant_type", "authorization_code"),
                    ]
                }

                fn request() -> MockReq {
                    Url::parse_with_params("https://localhost", params())
                        .unwrap()
                        .into()
                }

                #[test]
                fn it_sets_host_mock_clients_auth_uri() {
                    let mut storage = storage();
                    let req = request();
                    let resp = MockClient::handle_auth_redirect(false, req, &mut storage);
                    assert_that(&resp).is_ok();
                    let client = resp.unwrap();
                    assert_that(&client).has_code();
                }

                #[test]
                fn it_cleans_up_old_mock_client_when_state_is_used() {
                    let mut storage = storage();
                    let req = request();
                    let resp = MockClient::handle_auth_redirect(false, req, &mut storage);
                    assert_that(&resp).is_ok();
                    assert_that(&storage.get("MOCK_STATE")).is_err();
                }
            }

            mod when_state_is_not_in_storage {
                use super::*;
                use client::OauthClient;

                #[inline]
                fn storage() -> MemoryStorage {
                    MemoryStorage::new()
                }

                fn request() -> MockReq {
                    Url::parse_with_params("https://localhost", params())
                        .unwrap()
                        .into()
                }

                fn params() -> Vec<(&'static str, &'static str)> {
                    vec![
                        ("code", "MOCK_CODE"),
                        ("state", "MOCK_STATE"),
                        ("grant_type", "authorization_code"),
                    ]
                }

                #[test]
                fn it_returns_an_error() {
                    let mut storage = storage();
                    let req = request();
                    let resp = MockClient::handle_auth_redirect(false, req, &mut storage);
                    assert_that(&resp).is_err();
                }
            }

        }

    }
}

mod get_access_token_request {
    use super::*;
    mod code_grant_flow {
        use super::*;

        /// Specs out Happy Case for [Access Token Request](https://tools.ietf.org/html/rfc6749#section-4.1.3)
        mod happy_case {
            use super::*;
            use client::OauthClient;

            fn mock_client() -> MockClient {
                let mock_client = MockClient::new(
                    base_auth().with_secret(expected_client_secret()),
                    expected_redirect(),
                ).unwrap()
                    .with_state("MOCK_STATE")
                    .with_code("MOCK_CODE");

                assert_that(&mock_client.auth.client_id).is_equal_to(&expected_client_id().into());
                assert_that(&mock_client.auth.client_secret)
                    .is_equal_to(&Some(expected_client_secret().into()));

                assert_that(&mock_client.redirect_uri).is_equal_to(&expected_redirect().into());
                assert_that(&mock_client.auth.token_uri)
                    .is_equal_to(&Url::parse(expected_token_uri()).unwrap());
                return mock_client;
            }

            #[test]
            fn it_has_a_grant_type_in_params() {
                let client = mock_client();
                let request_result = client.get_access_token_request();
                assert_that(&request_result).is_ok();
                let request = request_result.unwrap();
                let params = UrlQueryParams::from(&request.url);
                assert_that(&params)
                    .has_param("grant_type")
                    .have_a_single_value()
                    .is_equal_to(Cow::from("authorization_code"));
            }

            #[test]
            fn it_has_a_code_in_params() {
                let client = mock_client();
                let request_result = client.get_access_token_request();
                assert_that(&request_result).is_ok();
                let request = request_result.unwrap();
                let params = UrlQueryParams::from(&request.url);
                assert_that(&params)
                    .has_param("code")
                    .have_a_single_value()
                    .is_equal_to(Cow::from("MOCK_CODE"));
            }

            #[test]
            fn it_has_a_redirect_url_in_params() {
                let client = mock_client();
                let request_result = client.get_access_token_request();
                assert_that(&request_result).is_ok();
                let request = request_result.unwrap();
                let params = UrlQueryParams::from(&request.url);
                assert_that(&params)
                    .has_param("redirect_uri")
                    .have_a_single_value()
                    .is_equal_to(Cow::from(expected_redirect()));
            }

            #[test]
            fn it_has_to_have_client_id_in_params() {
                let client = mock_client();
                let request_result = client.get_access_token_request();
                assert_that(&request_result).is_ok();
                let request = request_result.unwrap();
                let params = UrlQueryParams::from(&request.url);
                assert_that(&params)
                    .has_param("client_id")
                    .have_a_single_value()
                    .is_equal_to(Cow::from(expected_client_id()));
            }
        }

        mod without_code {
            use super::*;
            use client::OauthClient;
            use errors::ErrorKind;

            fn mock_client() -> MockClient {
                let mock_client = MockClient::new(base_auth(), expected_redirect())
                    .unwrap()
                    .with_state("MOCK_STATE");

                assert_that(&mock_client.auth.client_id).is_equal_to(&expected_client_id().into());
                assert_that(&mock_client.redirect_uri).is_equal_to(&expected_redirect().into());

                return mock_client;
            }

            #[test]
            fn it_has_a_grant_type_in_params() {
                let client = mock_client();
                let request_result = client.get_access_token_request();
                assert_that(&request_result)
                    .is_err()
                    .is_equal_to(&ErrorKind::msg(
                        "`code` was not set for token request. It is required for explciit flow",
                    ));
            }
        }

        mod with_secret {
            use super::*;
            use client::OauthClient;

            fn mock_client() -> MockClient {
                let mock_client = MockClient::new(
                    base_auth().with_secret(expected_client_secret()),
                    expected_redirect(),
                ).unwrap()
                    .with_state("MOCK_STATE")
                    .with_code("MOCK_CODE");

                assert_that(&mock_client.auth.client_secret)
                    .is_equal_to(&Some(expected_client_secret().into()));
                return mock_client;
            }

            #[test]
            fn it_has_a_secret_in_the_params() {
                let client = mock_client();
                let request_result = client.get_access_token_request();
                assert_that(&request_result).is_ok();
                let request = request_result.unwrap();
                let params = UrlQueryParams::from(&request.url);
                assert_that(&params)
                    .has_param("client_secret")
                    .have_a_single_value()
                    .is_equal_to(Cow::from(expected_client_secret()));
            }
        }

        mod without_secret {
            use super::*;
            use client::OauthClient;

            fn mock_client() -> MockClient {
                let mock_client = MockClient::new(base_auth(), expected_redirect())
                    .unwrap()
                    .with_state("MOCK_STATE")
                    .with_code("MOCK_CODE");

                return mock_client;
            }

            #[test]
            fn it_has_no_secret_in_the_params() {
                let client = mock_client();
                let request_result = client.get_access_token_request();
                assert_that(&request_result).is_ok();
                let request = request_result.unwrap();
                let params = UrlQueryParams::from(&request.url);
                assert_that(&params).has_no_param("client_secret");
            }
        }
    }

    mod implicit_flow {}
}

mod handle_token_response {
    use super::*;
    use client::responses::Token;
    use client::OauthClient;
    use serde_json;

    fn mock_client() -> MockClient {
        let mock_client = MockClient::new(base_auth(), expected_redirect())
            .unwrap()
            .with_state("MOCK_STATE")
            .with_code("MOCK_CODE");

        return mock_client;
    }

    fn base_auth() -> BaseAuthenticator {
        BaseAuthenticator::new(
            expected_client_id(),
            "http://example.com/auth",
            "http://example.com/token",
        ).unwrap()
            .with_secret("MOCK_SECRET")
    }

    fn token() -> String {
        serde_json::to_string_pretty(&Token::new("FAKE_TOKEN", "bearer")).unwrap()
    }

    #[test]
    fn it_adds_token_to_client() {
        let client = mock_client();
        let mut storage = storage();
        let resp = client.handle_token_response(token().into(), &mut storage);
        assert_that(&resp).is_ok();
    }
}
