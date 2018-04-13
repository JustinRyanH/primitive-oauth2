use client::MockReq;
use client::mock_client::MockClient;
use client::storage::MockMemoryStorage;
use spectral::prelude::*;

use errors::Result;

mod get_user_auth_request {
    use super::*;

    mod code_grant_flow {
        use super::*;
        use client::OauthClient;

        #[inline]
        fn storage() -> MockMemoryStorage {
            MockMemoryStorage::new()
        }

        #[inline]
        fn get_request(storage: &mut MockMemoryStorage) -> Result<MockReq> {
            MockClient::new().unwrap().get_user_auth_request(storage)
        }

        mod response_type {
            use super::*;

            #[test]
            fn it_sets_param_response_type_to_code() {
                let mut storage = storage();
                assert_that(&get_request(&mut storage)).is_ok();
            }
        }
        mod client_id {}
        mod redirect_uri {
            mod when_there_is_redirect {}
            mod when_there_is_no_redirect {}
        }
        mod scope {
            mod when_there_is_no_scope {}
            mod when_there_is_scope {}
        }
        mod state {
            mod when_state_is_on {}
            mod when_state_is_off {}
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
    mod when_happy {
        mod when_there_is_state {
            mod code {}
        }
        mod when_there_is_no_state {
            mod code {}
        }
    }
    mod when_error {}
}

mod get_access_token_request {
    mod when_valid_token_response {}
    mod when_not_valid_token_response {}
}
