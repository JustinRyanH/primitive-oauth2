use spectral::prelude::*;
use url::Url;

use super::auth::{auth as auth_route, auth_response};
use client::MockReq;
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
        MockServer::new().with_error(Error::msg("Server Error"))
    }

    #[test]
    fn returns_a_redirect_with_error() {
        let expected_req =
            MockReq::parse_error_req("https://example.com", &Error::msg("Server Error")).unwrap();
        assert_that(&auth_response(&server(), request(params())).redirect())
            .is_ok()
            .is_equal_to(expected_req);
    }
}
