use spectral::prelude::*;
use dotenv;
use envy;

use client::params::{ParamValue, UrlQueryParams};
use client::AccessType;
use client::authenticator::*;

mod given_an_authenticator {
    #[allow(unused_imports)]

    use std::iter::FromIterator;
    use super::*;

    mod get_auth_params {
        use super::*;
        pub fn subject() -> UrlQueryParams {
            dotenv::dotenv().expect("Failed to read the `.env` file");
            let auth = envy::prefixed("EXAMPLE_OAUTH2_")
                .from_env::<BaseAuthenticator>()
                .ok()
                .expect("Failed to Serialize BaseAuthenticator from .env");
            UrlQueryParams::from_iter(
                auth.get_auth_params(
                    "https://localhost:8000",
                    &vec!["example.profile", "example.email"]
                        .iter()
                        .map(|s| s.to_string())
                        .collect(),
                    AccessType::Grant,
                    "STATE",
                ),
            )
        }

        #[test]
        fn there_is_a_client_id_from_the_enviornment() {
            assert_that(&*subject())
                .contains_key("client_id".to_string())
                .is_equal_to(ParamValue::from("example_foobar_whatever@example.com"));
        }

        #[test]
        fn there_is_a_redirect_uri_from_the_enviornment() {
            assert_that(&*subject())
                .contains_key("redirect_uri".to_string())
                .is_equal_to(ParamValue::from("https://localhost:8000"));
        }

        #[test]
        fn there_are_some_scopes_from_the_enviornment() {
            assert_that(&*subject())
                .contains_key("scope".to_string())
                .is_equal_to(ParamValue::from_iter(vec![
                    "example.profile",
                    "example.email",
                ]));
        }
    }
}
