use spectral::prelude::*;
use url::Url;

use super::auth::auth as auth_route;
use client::mock_client::MockReq;
use client::mock_server::*;
use errors::Error;

fn server() -> MockServer {
    MockServer::new()
}

fn request(params: Vec<(&'static str, &'static str)>) -> MockReq {
    MockReq {
        url: Url::parse_with_params("https://example.net/auth", params).unwrap(),
        body: "".to_string(),
    }
}

mod happy_case {
    use super::*;

    fn params() -> Vec<(&'static str, &'static str)> {
        vec![
            ("response_type", "code"),
            ("client_id", "someid@example.com"),
            ("redirect_uri", "https://localhost:8080/oauth/example"),
            ("scope", "api.example.com/user.profile"),
            ("scope", "api.example.com/add_item"),
            ("state", "MOCK_STATE"),
        ]
    }

    #[test]
    fn returns_a_redirect() {
        let redirect_req: MockReq = Url::parse_with_params(
            "https://localhost/example/auth",
            [("state", "MOCK_STATE"), ("code", "MOCK_CODE")].iter(),
        ).unwrap()
            .into();
        assert_that(&auth_route(&server(), &request(params())))
            .is_ok()
            .is_equal_to(redirect_req);
    }
}

mod with_error {
    use super::*;

    fn params() -> Vec<(&'static str, &'static str)> {
        vec![]
    }

    fn server() -> MockServer {
        MockServer::new().with_error(Error::invalid_request(
            None,
            Some("https://doc.example.net/invalid_request"),
        ))
    }

    #[test]
    fn returns_a_redirect_with_error() {
        let expected_err = Error::invalid_request(Some("Bad Request: Missing `state`"), None);
        assert_that(&auth_route(&server(), &request(params())))
            .is_err()
            .is_equal_to(expected_err);
    }
}

/// 4.1.1 client_id: REQUIRED.  The client identifier as described in Section 2.2.
mod client_id_param {
    use super::*;

    mod when_bad {
        use super::*;

        fn params() -> Vec<(&'static str, &'static str)> {
            vec![
                ("response_type", "code"),
                ("client_id", "example.com"),
                ("redirect_uri", "https://localhost:8080/oauth/example"),
                ("scope", "api.example.com/user.profile"),
                ("scope", "api.example.com/add_item"),
                ("state", "MOCK_STATE"),
            ]
        }

        #[test]
        fn it_returns_400_response() {
            let expected_err =
                Error::unauthorized_client(Some("Unauthorized: Client Not Authorized"), None);
            assert_that(&auth_route(&server(), &request(params())))
                .is_err()
                .is_equal_to(expected_err);
        }
    }

    mod when_missing {
        use super::*;

        fn params() -> Vec<(&'static str, &'static str)> {
            vec![
                ("response_type", "code"),
                ("redirect_uri", "https://localhost:8080/oauth/example"),
                ("scope", "api.example.com/user.profile"),
                ("scope", "api.example.com/add_item"),
                ("state", "MOCK_STATE"),
            ]
        }

        fn request() -> MockReq {
            MockReq {
                url: Url::parse_with_params("https://example.net/auth", params()).unwrap(),
                body: "".to_string(),
            }
        }

        #[test]
        fn it_returns_400_response() {
            let expected_err =
                Error::invalid_request(Some("Bad Request: Missing `client_id`"), None);
            assert_that(&auth_route(&server(), &request()))
                .is_err()
                .is_equal_to(expected_err);
        }
    }
}

/// 4.1.1 redirect_uri OPTIONAL. [As described in Section 3.1.2.](https://tools.ietf.org/html/rfc6749#section-3.1.2)
mod redirect_uri_param {
    use super::*;

    mod when_reqired_and_missing {
        use super::*;

        fn server() -> MockServer {
            MockServer::new().require_redirect()
        }

        mod and_missing {
            use super::*;

            fn params() -> Vec<(&'static str, &'static str)> {
                vec![
                    ("response_type", "code"),
                    ("client_id", "someid@example.com"),
                    ("scope", "api.example.com/user.profile"),
                    ("scope", "api.example.com/add_item"),
                    ("state", "MOCK_STATE"),
                ]
            }

            #[test]
            /// Returns [4.1.2.1. Error Response](https://tools.ietf.org/html/rfc6749#section-4.1.2.1)
            /// with an invalid request
            fn it_returns_a_redirect_with_error() {
                let expected_err =
                    Error::invalid_request(Some("Bad Request: Missing `redirect_uri`"), None);
                assert_that(&auth_route(&server(), &request(params())))
                    .is_err()
                    .is_equal_to(expected_err);
            }
        }

        mod not_in_validation_list {
            use super::*;

            fn server() -> MockServer {
                MockServer::new().require_redirect()
            }

            fn params() -> Vec<(&'static str, &'static str)> {
                vec![
                    ("response_type", "code"),
                    ("client_id", "someid@example.com"),
                    ("redirect_uri", "https://localhost:8080/oauth/bad"),
                    ("scope", "api.example.com/user.profile"),
                    ("scope", "api.example.com/add_item"),
                    ("state", "MOCK_STATE"),
                ]
            }

            #[test]
            fn it_returns_a_redirect_with_error() {
                let expected_err = Error::invalid_request(
                    Some("Bad Request: Redirect Uri does not match valid uri"),
                    None,
                );
                assert_that(&auth_route(&server(), &request(params())))
                    .is_err()
                    .is_equal_to(expected_err);
            }
        }
    }

    mod when_not_reqired_and_missing {
        use super::*;

        fn params() -> Vec<(&'static str, &'static str)> {
            vec![
                ("response_type", "code"),
                ("client_id", "someid@example.com"),
                ("scope", "api.example.com/user.profile"),
                ("scope", "api.example.com/add_item"),
                ("state", "MOCK_STATE"),
            ]
        }

        #[test]
        fn returns_a_redirect() {
            assert_that(&auth_route(&server(), &request(params()))).is_ok();
        }
    }
}

/// 4.1.1 scope OPTIONAL.  The scope of the access request [as described by Section 3.3.](https://tools.ietf.org/html/rfc6749#section-3.3)
mod scope_param {
    use super::*;
    mod when_missing {
        use super::*;

        fn params() -> Vec<(&'static str, &'static str)> {
            vec![
                ("response_type", "code"),
                ("client_id", "someid@example.com"),
                ("redirect_uri", "https://localhost:8080/oauth/example"),
                ("state", "MOCK_STATE"),
            ]
        }

        #[test]
        fn returns_a_redirect() {
            assert_that(&auth_route(&server(), &request(params()))).is_ok();
        }
    }
    mod when_bad {
        use super::*;
        fn params() -> Vec<(&'static str, &'static str)> {
            vec![
                ("response_type", "code"),
                ("client_id", "someid@example.com"),
                ("redirect_uri", "https://localhost:8080/oauth/example"),
                ("scope", "api.example.com/fasfa"),
                ("state", "MOCK_STATE"),
            ]
        }

        fn server() -> MockServer {
            MockServer::new().require_redirect()
        }

        #[test]
        fn it_returns_a_redirect_with_error() {
            let expected_err = Error::invalid_request(
                None,
                Some("https://docs.example.com/scopes?invalid_scope=api.example.com/fasfa"),
            );
            assert_that(&auth_route(&server(), &request(params())))
                .is_err()
                .is_equal_to(expected_err);
        }
    }
}

/// 4.1.1 state RECOMMENDED.
/// An opaque value used by the client to maintain state between the request and callback.
/// The authorization server includes this value when redirecting the user-agent back
/// to the client.  The parameter SHOULD be used for preventing
/// cross-site request forgery [as described in Section 10.12.](https://tools.ietf.org/html/rfc6749#section-10.12)
mod state_param {
    use super::*;

    mod when_required_and_missing {
        use super::*;

        fn params() -> Vec<(&'static str, &'static str)> {
            vec![
                ("response_type", "code"),
                ("client_id", "someid@example.com"),
                ("redirect_uri", "https://localhost:8080/oauth/example"),
            ]
        }

        #[test]
        fn it_returns_a_redirect_with_error() {
            let expected_err = Error::invalid_request(Some("Bad Request: Missing `state`"), None);
            assert_that(&auth_route(&server(), &request(params())))
                .is_err()
                .is_equal_to(expected_err);
        }
    }

    mod when_not_required_and_missing {
        use super::*;

        fn params() -> Vec<(&'static str, &'static str)> {
            vec![
                ("response_type", "code"),
                ("client_id", "someid@example.com"),
                ("redirect_uri", "https://localhost:8080/oauth/example"),
            ]
        }

        #[test]
        fn it_returns_a_redirect_with_error() {
            let expected_err = Error::invalid_request(Some("Bad Request: Missing `state`"), None);
            assert_that(&auth_route(&server(), &request(params())))
                .is_err()
                .is_equal_to(expected_err);
        }
    }
}
