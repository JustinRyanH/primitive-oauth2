#[allow(unused_imports)]

use std::iter::FromIterator;

#[allow(unused_imports)]
use futures::Future;
use futures_cpupool::CpuPool;

use client::storage::MockMemoryStorage;
use client::params::{ParamValue, UrlQueryParams};
use client::mock_client::*;
use client::mock_server::*;
use errors::Result;

mod given_mock_client {
    #[allow(unused_imports)]

    use client::OauthClient;
    use spectral::prelude::*;
    use super::*;

    #[derive(Debug, Clone)]
    struct Env {
        pool: CpuPool,
        storage: MockMemoryStorage,
    }

    fn env() -> Env {
        Env {
            pool: CpuPool::new(1),
            storage: MockMemoryStorage::new(),
        }
    }

    mod get_user_auth_request {
        use super::*;

        fn subject() -> UrlQueryParams {
            let env = env();
            let subject_of_interest = MockClient::new()
                .unwrap()
                .get_user_auth_request(&mut env.storage.clone());

            env.pool
                .clone()
                .spawn(subject_of_interest)
                .wait()
                .unwrap()
                .into()
        }

        #[test]
        fn request_contains_client_id() {
            assert_that(&*subject())
                .contains_key("client_id".to_string())
                .is_equal_to(ParamValue::from("someid@example.com"));
        }

        #[test]
        fn request_contains_redirect_uri() {
            assert_that(&*subject())
                .contains_key("redirect_uri".to_string())
                .is_equal_to(ParamValue::from("https://localhost/auth"));
        }

        #[test]
        fn request_contains_scope() {
            assert_that(&*subject())
                .contains_key("scope".to_string())
                .is_equal_to(ParamValue::from_iter(vec![
                    "api.example.com/user.profile",
                    "api.example.com/user.me",
                ]));
        }

        #[test]
        fn request_contains_response_type() {
            assert_that(&*subject())
                .contains_key("response_type".to_string())
                .is_equal_to(ParamValue::from("code"));
        }

        #[test]
        fn request_contains_state() {
            assert_that(&*subject())
                .contains_key("state".to_string())
                .is_equal_to(ParamValue::from("EXAMPLE_STATE"));
        }
    }

    mod handle_auth_request {
        use super::*;

        #[allow(unused_imports)]
        use client::mock_client::test_helpers::MockClientHelper;
        use client::OauthClient;

        fn subject(server: MockServer) -> Result<MockClient> {
            let env = env();
            let storage = env.storage.clone();
            let subject_of_interest = MockClient::new()
                .unwrap()
                .get_user_auth_request(&mut storage.clone())
                .and_then(move |req| server.redirect(req))
                .and_then(move |req| MockClient::handle_auth_request(req, &mut storage.clone()));

            env.clone().pool.clone().spawn(subject_of_interest).wait()
        }

        mod when_there_is_a_previous_state {
            use super::*;
            fn server() -> MockServer {
                MockServer::new()
            }

            #[test]
            fn it_returns_client_with_code_from_server() {
                assert_that(&subject(server()))
                    .is_ok()
                    .has_code()
                    .is_equal_to("MOCK_CODE".to_string());
            }
        }

        // TODO: When there was no previous state found

        // TODO: When State isn't used
    }

    mod request_token {
        use super::*;

        mod with_code {
            use super::*;

            fn subject() -> Result<MockResp> {
                let env = env();
                let subject_of_interest = MockClient::new()
                    .unwrap()
                    .with_code("MOCK_CODE")
                    .request_token();

                env.clone().pool.clone().spawn(subject_of_interest).wait()
            }

            #[test]
            fn it_returns_a_response_with_a_token() {
                assert_that(&subject()).is_ok();
            }
        }

    }

    mod handle_token_response {
        mod it_stores_successful_client {}
    }
}
