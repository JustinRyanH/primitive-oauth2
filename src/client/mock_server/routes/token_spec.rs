use spectral::prelude::*;
use url::Url;

use super::token::{token as token_route, token_response, MOCK_TOKEN};
use client::TokenResponse;
use client::mock_client::{MockReq, MockResp};
use client::mock_server::*;
use errors::Error;

fn server() -> MockServer {
    MockServer::new()
}

fn request(params: Vec<(&'static str, &'static str)>) -> MockReq {
    MockReq {
        url: Url::parse_with_params("https://example.net/token", params).unwrap(),
        body: "".to_string(),
    }
}

mod happy_case {
    use super::*;

    fn params() -> Vec<(&'static str, &'static str)> {
        vec![("code", "MOCK_CODE"), ("state", "MOCK_STATE")]
    }

    #[test]
    fn returns_a_response() {
        let expected_resp =
            MockResp::parse_access_token_response(&TokenResponse::new(MOCK_TOKEN, "bearer"))
                .unwrap();

        assert_that(&token_route(&server(), &request(params())))
            .is_ok()
            .is_equal_to(expected_resp);
    }

    mod server_with_scope {
        use super::*;

        fn server() -> MockServer {
            MockServer::new().with_scope(vec!["user.foo", "user.profile"])
        }

        #[test]
        fn returns_a_response() {
            let expected_resp =
                MockResp::parse_access_token_response(&TokenResponse::new(MOCK_TOKEN, "bearer")
                    .with_scope(&vec!["user.foo", "user.profile"]))
                    .unwrap();

            assert_that(&token_route(&server(), &request(params())))
                .is_ok()
                .is_equal_to(expected_resp);
        }

    }
    mod server_with_ttl {
        use super::*;

        fn server() -> MockServer {
            MockServer::new().with_expiration(3600)
        }

        #[test]
        fn returns_a_response() {
            let expected_resp = MockResp::parse_access_token_response(&TokenResponse::new(
                MOCK_TOKEN,
                "bearer",
            ).with_expiration(3600))
                .unwrap();

            assert_that(&token_route(&server(), &request(params())))
                .is_ok()
                .is_equal_to(expected_resp);
        }

    }
}

mod errors {
    use super::*;

    fn server() -> MockServer {
        MockServer::new().with_error(Error::msg("Server Error"))
    }

    fn params() -> Vec<(&'static str, &'static str)> {
        vec![
            ("response_type", "token"),
            ("client_id", "someid@example.com"),
            ("client_secret", "MOCK_SECRET"),
            ("redirect_uri", "https://localhost:8080/oauth/example"),
            ("state", "MOCK_STATE"),
        ]
    }

    #[test]
    fn returns_a_response_error() {
        let expected_resp = MockResp::parse_error_resp(&"Server Error".into()).unwrap();

        assert_that(&token_response(&server(), request(params())).response())
            .is_ok()
            .is_equal_to(expected_resp);
    }
}
