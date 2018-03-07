extern crate async_oauth2;

extern crate dotenv;
extern crate envy;
#[cfg(test)]
extern crate rspec;

use rspec::given;

use async_oauth2::Authenticator;
/*
export EXAMPLE_OAUTH2_CLIENT_SECRET="super_secret"
export EXAMPLE_OAUTH2_AUTH_URI="https://example.com/v1/auth"
export EXAMPLE_OAUTH2_TOKEN_URI="https://example.com/v1/token"
*/

#[test]
fn explicit_flow() {
    dotenv::dotenv().expect("Failed to read the `.env` file");

    rspec::run(&given(
        "a Oauth2 Common Flow",
        Authenticator::default(),
        |ctx| {
            ctx.context(
                "When the Authenticator is loaded from enviornment variables",
                |ctx| {
                    ctx.before_each(|env| {
                        *env = envy::prefixed("EXAMPLE_OAUTH2_")
                            .from_env::<Authenticator>()
                            .ok()
                            .expect("Failed to Serialize Authenticator from .env");
                    });

                    ctx.it(
                        "then creates a Authenticator Object with client id",
                        |env| {
                            let expected_client_id = "example_foobar_whatever@example.com";
                            let actual_client_id = env.get_client_id();
                            assert_eq!(
                                actual_client_id,
                                expected_client_id,
                                "Expected Authenticator's client_id to eq {}, but got {}",
                                expected_client_id,
                                actual_client_id
                            );
                        },
                    );

                    ctx.it(
                        "then creates an Authenticator Object with client secret",
                        |env| {
                            let expected_client_secret = "super_secret";
                            let actual_client_secret = env.get_client_secret();
                            assert_eq!(
                                actual_client_secret,
                                expected_client_secret,
                                "Expected Authenticator's client_secret to eq {}, but got {}",
                                expected_client_secret,
                                actual_client_secret
                            );
                        },
                    )
                },
            );
        },
    ));
}
