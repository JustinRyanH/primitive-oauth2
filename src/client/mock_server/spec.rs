mod describe_mock_sever {
    use spectral::prelude::*;
    use url::Url;

    use client::TokenResponse;
    use client::mock_client::{MockReq, MockResp};
    use client::mock_server::*;

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
