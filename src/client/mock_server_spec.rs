mod describe_mock_sever {
    use url::Url;
    use spectral::prelude::*;

    use client::mock_client::{MockReq, MockResp};
    use client::mock_server::*;

    fn server() -> MockServer {
        MockServer::new()
    }

    mod route_token {}

    /// Used Simulate the [4.1.1.  Authorization Request](https://tools.ietf.org/html/rfc6749#section-4.1.1)
    /// request, and the expected responses if they failed.
    mod route_auth {
        use super::*;

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

            fn request() -> MockReq {
                MockReq {
                    url: Url::parse_with_params("https://example.net/auth", params()).unwrap(),
                    body: "".to_string(),
                }
            }

            #[test]
            fn returns_a_redirect() {
                assert_that(&server().send_request(request()).redirect()).is_ok();
            }
        }

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

                fn request() -> MockReq {
                    MockReq {
                        url: Url::parse_with_params("https://example.net/auth", params()).unwrap(),
                        body: "".to_string(),
                    }
                }

                #[test]
                fn it_returns_400_response() {
                    let expected_resp: MockResp = "Bad Request: Invalid `client_id`".into();
                    assert_that(&server().send_request(request()).response())
                        .is_ok()
                        .is_equal_to(expected_resp);
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
                    let expected_resp: MockResp = "Bad Request: Missing `client_id`".into();
                    assert_that(&server().send_request(request()).response())
                        .is_ok()
                        .is_equal_to(expected_resp);
                }
            }
        }

        mod redirect_uri_param {
            use super::*;

            mod when_not_uri {
                use super::*;

                fn params() -> Vec<(&'static str, &'static str)> {
                    vec![
                        ("response_type", "code"),
                        ("client_id", "someid@example.com"),
                        ("redirect_uri", "/oauth/example"),
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
                fn it_returns_a_400_response() {
                    let expected_resp: MockResp =
                        "Bad Request: Invalid Url for `redirect_url`".into();
                    assert_that(&server().send_request(request()).response())
                        .is_ok()
                        .is_equal_to(expected_resp);
                }
            }

            mod when_reqired_and_missing {
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

                fn request() -> MockReq {
                    MockReq {
                        url: Url::parse_with_params("https://example.net/auth", params()).unwrap(),
                        body: "".to_string(),
                    }
                }

                #[test]
                fn it_returns_a_response_with_error() {
                    let expected_resp: MockResp = "Bad Request: Missing `redirect_uri`".into();
                    assert_that(&server()
                        .require_redirect()
                        .send_request(request())
                        .response())
                        .is_ok()
                        .is_equal_to(expected_resp);
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

                fn request() -> MockReq {
                    MockReq {
                        url: Url::parse_with_params("https://example.net/auth", params()).unwrap(),
                        body: "".to_string(),
                    }
                }

                #[test]
                fn returns_a_redirect() {
                    assert_that(&server().send_request(request()).redirect()).is_ok();
                }
            }
        }

        mod scope_param {

            mod when_missing {}
            mod when_valid {}
            mod when_bad {}
        }

        mod state_param {
            use super::*;

            mod when_required {
                use super::*;

                fn params() -> Vec<(&'static str, &'static str)> {
                    vec![
                        ("response_type", "code"),
                        ("client_id", "someid@example.com"),
                        ("redirect_uri", "https://localhost:8080/oauth/example"),
                    ]
                }

                fn request() -> MockReq {
                    MockReq {
                        url: Url::parse_with_params("https://example.net/auth", params()).unwrap(),
                        body: "".to_string(),
                    }
                }

                #[test]
                fn it_returns_a_response_with_error() {
                    let expected_resp: MockResp =
                        "Bad Request: State should be optional, but it currently is not".into();
                    // It is a Response
                    assert_that(&server().send_request(request()).response())
                        .is_ok()
                        .is_equal_to(expected_resp);
                }

            }
            mod when_not_required {}
        }
    }

    mod no_route {

        #[test]
        #[should_panic]
        fn it_returns_404_response() {
            unimplemented!()
        }
    }
}
