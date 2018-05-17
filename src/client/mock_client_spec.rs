use std::borrow::Cow;

#[allow(unused_imports)]
use assertions::*;
use spectral::prelude::*;

use url::Url;

use client::authenticator::BaseAuthenticator;
use client::mock_client::MockClient;
use client::params::UrlQueryParams;
use client::requests::MockReq;
use client::storage::MemoryStorage;

#[inline]
fn storage() -> MemoryStorage {
    MemoryStorage::new()
}

#[inline]
fn base_auth() -> BaseAuthenticator {
    BaseAuthenticator::new(
        "someid@example.com",
        "test",
        "http://example.com/auth",
        "http://example.com/token",
    ).unwrap()
}

#[inline]
fn expected_redirect() -> &'static str {
    "https://localhost:8080/oauth/example"
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
    use client::OauthClient;

    mod when_happy {
        use super::*;

        mod when_there_is_state {
            /// Assertions
            /// When Code is not supplied -> Fail with Invalid Request
            /// When Client w/ State is stored -> Get existing State -> Assert it is Removed
            /// When Client w/ State is not stored -> Fail with an InvalidState
            use super::*;
            use client::ClientStorage;

            #[inline]
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
        }

    }
    mod when_error {}
}

mod get_access_token_request {
    mod when_valid_token_response {}
    mod when_not_valid_token_response {}
}
