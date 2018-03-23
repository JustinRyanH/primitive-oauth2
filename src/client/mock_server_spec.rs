mod describe_mock_sever {
    use url::Url;
    use spectral::prelude::*;

    use client::FutResult;
    use client::mock_client::{MockReq, MockResp};
    use client::mock_server::*;

    mod route_token {}

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

            fn server() -> MockServer {
                MockServer::new()
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

            mod when_bad {}
            mod when_missing {}
        }

        mod redirect_uri_param {
            use super::*;

            mod when_not_uri {}
            mod when_missing {}
        }

        mod scope_param {
            use super::*;

            mod when_missing {}
            mod when_valid {}
            mod when_bad {}
        }

        mod state_param {
            use super::*;

            mod when_required {}
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
