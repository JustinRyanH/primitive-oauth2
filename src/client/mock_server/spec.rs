mod describe_mock_sever {
    use spectral::prelude::*;
    use url::Url;

    use client::TokenResponse;
    use client::mock_client::{MockReq, MockResp};
    use client::mock_server::*;
    use errors::Error;

    fn server() -> MockServer {
        MockServer::new()
    }

    mod route_token {
        use super::*;

        mod happy_case {
            use super::*;

            fn params() -> Vec<(&'static str, &'static str)> {
                vec![
                    ("response_type", "token"),
                    ("client_id", "someid@example.com"),
                    ("client_secret", "MOCK_SECRET"),
                    ("redirect_uri", "https://localhost:8080/oauth/example"),
                    ("state", "MOCK_STATE"),
                ]
            }

            fn request() -> MockReq {
                MockReq {
                    url: Url::parse_with_params("https://example.net/token", params()).unwrap(),
                    body: "".to_string(),
                }
            }

            #[test]
            fn returns_a_response() {
                let expected_resp = MockResp::parse_access_token_response(&TokenResponse::new(
                    "2YotnFZFEjr1zCsicMWpAA",
                    "bearer",
                )).unwrap();

                assert_that(&server().send_request(request()).response())
                    .is_ok()
                    .is_equal_to(expected_resp);
            }

            mod server_with_state {}
            mod server_with_scope {}
            mod server_with_ttl {}
        }

        mod errors {
            use super::*;

            fn params() -> Vec<(&'static str, &'static str)> {
                vec![
                    ("response_type", "token"),
                    ("client_id", "someid@example.com"),
                    ("client_secret", "MOCK_SECRET"),
                    ("redirect_uri", "https://localhost:8080/oauth/example"),
                    ("state", "MOCK_STATE"),
                ]
            }

            fn request() -> MockReq {
                MockReq {
                    url: Url::parse_with_params("https://example.net/token", params()).unwrap(),
                    body: "".to_string(),
                }
            }

            #[test]
            fn returns_a_response_error() {
                let expected_resp = MockResp::parse_error_resp(&"Server Error".into()).unwrap();

                assert_that(&server()
                    .with_error("Server Error")
                    .send_request(request())
                    .response())
                    .is_ok()
                    .is_equal_to(expected_resp);
            }
        }
    }

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
            fn request() -> MockReq {
                MockReq {
                    url: Url::parse_with_params("https://example.net/auth", params()).unwrap(),
                    body: "".to_string(),
                }
            }

            #[test]
            fn returns_a_redirect_with_error() {
                let expected_req: MockReq = Url::parse_with_params(
                    "https://example.com",
                    vec![
                        ("error", "invalid_request"),
                        ("error_uri", "https://doc.example.net/invalid_request"),
                    ],
                ).unwrap()
                    .into();
                assert_that(&server().send_request(request()).redirect())
                    .is_ok()
                    .is_equal_to(expected_req);
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

                fn request() -> MockReq {
                    MockReq {
                        url: Url::parse_with_params("https://example.net/auth", params()).unwrap(),
                        body: "".to_string(),
                    }
                }

                #[test]
                fn it_returns_400_response() {
                    let expected_req: MockReq = Url::parse_with_params(
                        "https://example.com",
                        vec![
                            ("error", "unauthorized_client"),
                            ("error_description", "Unauthorized: Client Not Authorized"),
                        ],
                    ).unwrap()
                        .into();
                    assert_that(&server().send_request(request()).redirect())
                        .is_ok()
                        .is_equal_to(expected_req);
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
                    let expected_req: MockReq = Url::parse_with_params(
                        "https://example.com",
                        vec![
                            ("error", "invalid_request"),
                            ("error_description", "Bad Request: Missing `client_id`"),
                        ],
                    ).unwrap()
                        .into();

                    assert_that(&server().send_request(request()).redirect())
                        .is_ok()
                        .is_equal_to(expected_req);
                }
            }
        }

        /// 4.1.1 redirect_uri OPTIONAL. [As described in Section 3.1.2.](https://tools.ietf.org/html/rfc6749#section-3.1.2)
        mod redirect_uri_param {
            use super::*;

            mod when_reqired_and_missing {
                use super::*;

                fn request(params: Vec<(&'static str, &'static str)>) -> MockReq {
                    MockReq {
                        url: Url::parse_with_params("https://example.net/auth", params).unwrap(),
                        body: "".to_string(),
                    }
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
                        let expected_req: MockReq = Url::parse_with_params(
                            "https://example.com",
                            vec![
                                ("error", "invalid_request"),
                                ("error_description", "Bad Request: Missing `redirect_uri`"),
                            ],
                        ).unwrap()
                            .into();
                        assert_that(&server()
                            .require_redirect()
                            .send_request(request(params()))
                            .redirect())
                            .is_ok()
                            .is_equal_to(expected_req);
                    }
                }

                mod not_in_validation_list {
                    use super::*;

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
                        let expected_req: MockReq = Url::parse_with_params(
                            "https://example.com",
                            vec![
                                ("error", "invalid_request"),
                                (
                                    "error_description",
                                    "Bad Request: Redirect Uri does not match valid uri",
                                ),
                            ],
                        ).unwrap()
                            .into();
                        assert_that(&server()
                            .require_redirect()
                            .send_request(request(params()))
                            .redirect())
                            .is_ok()
                            .is_equal_to(expected_req);
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

                fn request() -> MockReq {
                    MockReq {
                        url: Url::parse_with_params("https://example.net/auth", params()).unwrap(),
                        body: "".to_string(),
                    }
                }

                #[test]
                fn it_returns_a_redirect_with_error() {
                    let expected_req: MockReq = Url::parse_with_params(
                        "https://example.com",
                        vec![
                            ("error", "invalid_request"),
                            ("error_uri", "https://docs.example.com/scopes?invalid_scope=api.example.com/fasfa"),
                        ],
                    ).unwrap()
                        .into();
                    assert_that(&server()
                        .require_redirect()
                        .send_request(request())
                        .redirect())
                        .is_ok()
                        .is_equal_to(expected_req);
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

                fn request() -> MockReq {
                    MockReq {
                        url: Url::parse_with_params("https://example.net/auth", params()).unwrap(),
                        body: "".to_string(),
                    }
                }

                #[test]
                fn it_returns_a_redirect_with_error() {
                    let expected_req: MockReq = Url::parse_with_params(
                        "https://example.com",
                        vec![
                            ("error", "invalid_request"),
                            ("error_description", "Bad Request: Missing `state`"),
                        ],
                    ).unwrap()
                        .into();
                    // It is a Response
                    assert_that(&server().send_request(request()).redirect())
                        .is_ok()
                        .is_equal_to(expected_req);
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

                fn request() -> MockReq {
                    MockReq {
                        url: Url::parse_with_params("https://example.net/auth", params()).unwrap(),
                        body: "".to_string(),
                    }
                }

                #[test]
                fn it_returns_a_redirect_with_error() {
                    let expected_req: MockReq = Url::parse_with_params(
                        "https://example.com",
                        vec![
                            ("error", "invalid_request"),
                            ("error_description", "Bad Request: Missing `state`"),
                        ],
                    ).unwrap()
                        .into();
                    // It is a Response
                    assert_that(&server().send_request(request()).redirect())
                        .is_ok()
                        .is_equal_to(expected_req);
                }
            }
        }
    }

    mod no_route {
        use super::*;

        fn request() -> MockReq {
            MockReq {
                url: Url::parse("https://example.net/").unwrap(),
                body: "".to_string(),
            }
        }

        #[test]
        fn it_returns_404_response() {
            let expected_resp: MockResp = "{\
                                           \"error\":\"server_error\",\
                                           \"error_description\":\"404: Route not found\"\
                                           }"
                .into();
            assert_that(&server().send_request(request()).response())
                .is_ok()
                .is_equal_to(expected_resp);
        }
    }
}
