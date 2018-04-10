mod get_user_auth_request {
    mod code_grant_flow {
        mod response_type {}
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
