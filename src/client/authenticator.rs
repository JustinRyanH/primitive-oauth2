use url_serde;
use url::Url;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseAuthenticator {
    client_id: String,
    client_secret: String,
    #[serde(with = "url_serde")] auth_uri: Url,
    #[serde(with = "url_serde")] token_uri: Url,
}

impl BaseAuthenticator {
    pub fn new<S: Into<String>>(
        client_id: S,
        client_secret: S,
        auth_uri: Url,
        token_uri: Url,
    ) -> BaseAuthenticator {
        BaseAuthenticator {
            client_id: client_id.into(),
            client_secret: client_secret.into(),
            auth_uri,
            token_uri,
        }
    }
}

impl Default for BaseAuthenticator {
    fn default() -> Self {
        BaseAuthenticator {
            client_id: Default::default(),
            client_secret: Default::default(),
            auth_uri: Url::parse("http://localhost/auth").unwrap(),
            token_uri: Url::parse("http://localhost/token").unwrap(),
        }
    }
}

impl BaseAuthenticator {
    pub fn get_client_id(&self) -> &str {
        self.client_id.as_ref()
    }

    pub fn get_client_secret(&self) -> &str {
        self.client_secret.as_ref()
    }

    pub fn get_auth_uri(&self) -> &str {
        self.auth_uri.as_ref()
    }

    pub fn get_token_uri(&self) -> &str {
        self.token_uri.as_ref()
    }

    pub fn get_auth_params(
        &self,
        redirect_uri: &str,
        scopes: &Vec<String>,
    ) -> Vec<(&str, String)> {
        let parsed_scopes: Vec<(&str, String)> = scopes.into_iter().map(|scope| ("scope", scope.clone())).collect();
        vec![
            ("client_id", self.client_id.clone()),
            ("redirect_uri", String::from(redirect_uri)),
        ].into_iter()
            .chain(parsed_scopes)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use rspec::{self, given};
    use dotenv;
    use envy;

    #[test]
    fn authenticator_is_serializable() {
        dotenv::dotenv().expect("Failed to read the `.env` file");
        rspec::run(&given(
            "An BaseAuthenticator",
            BaseAuthenticator::default(),
            |ctx| {
                ctx.context(
                    "When creating BaseAuthenticator with envy and/or another serde serializer",
                    |ctx| {
                        ctx.before_each(|env| {
                            *env = envy::prefixed("EXAMPLE_OAUTH2_")
                                .from_env::<BaseAuthenticator>()
                                .ok()
                                .expect("Failed to Serialize BaseAuthenticator from .env");
                        });

                        ctx.context("#get_auth_params", |ctx| {
                            ctx.it("pushes the client_id in the params", |env| {
                                let result = env.get_auth_params("", &vec![]);
                                assert!(result.contains(&(
                                    "client_id",
                                    String::from("example_foobar_whatever@example.com"),
                                )));
                            });

                            ctx.it("pushes the redirect_uri into params", |env| {
                                let result = env.get_auth_params("https://localhost:8000", &vec![]);
                                assert!(result.contains(&(
                                    "redirect_uri",
                                    String::from("https://localhost:8000"),
                                )));
                                assert_eq!(result.len(), 2);
                            });

                            ctx.it("pushes the scopes into params", |env| {
                                let result = env.get_auth_params(
                                    "https:://localhost:8080",
                                    &vec![
                                        "user.profile".to_string(),
                                        "user.openid".to_string(),
                                    ],
                                );
                                assert!(result.contains(&("scope", String::from("user.profile"),)));
                                assert!(result.contains(&("scope", String::from("user.openid"),)));
                            });
                        });

                        ctx.context("BaseAuthenticator Attributes", |ctx| {
                            ctx.it(
                                "then creates an BaseAuthenticator Object with a client id",
                                |env| {
                                    let expected_client_id = "example_foobar_whatever@example.com";
                                    let actual_client_id = env.get_client_id();
                                    assert_eq!(
                                        actual_client_id, expected_client_id,
                                        "Expected BaseAuthenticator's client_id to eq {}, but got {}",
                                        expected_client_id, actual_client_id
                                    );
                                },
                            );

                            ctx.it(
                                "then creates an BaseAuthenticator Object with a client secret",
                                |env| {
                                    let expected_client_secret = "super_secret";
                                    let actual_client_secret = env.get_client_secret();
                                    assert_eq!(
                                    actual_client_secret, expected_client_secret,
                                    "Expected BaseAuthenticator's client_secret to eq {}, but got {}",
                                    expected_client_secret, actual_client_secret
                                );
                                },
                            );

                            ctx.it(
                                "then creates an BaseAuthenticator Object with a auth uri",
                                |env| {
                                    let expected_auth_uri = "https://example.com/v1/auth";
                                    let actual_auth_uri = env.get_auth_uri();
                                    assert_eq!(
                                        actual_auth_uri, expected_auth_uri,
                                        "Expected BaseAuthenticator's auth_uri to eq {}, but got {}",
                                        expected_auth_uri, actual_auth_uri
                                    );
                                },
                            );

                            ctx.it(
                                "then creates an BaseAuthenticator Object with a token uri",
                                |env| {
                                    let expected_token_uri = "https://example.com/v1/token";
                                    let actual_token_uri = env.get_token_uri();
                                    assert_eq!(
                                        actual_token_uri, expected_token_uri,
                                        "Expected BaseAuthenticator's token_uri to eq {}, but got {}",
                                        expected_token_uri, actual_token_uri
                                    );
                                },
                            );
                        });
                    },
                );
            },
        ));
    }
}
