use errors::OAuthResult;
use url::Url;
use url_serde;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BaseAuthenticator {
    pub client_id: String,
    pub client_secret: Option<String>,
    #[serde(with = "url_serde")]
    pub auth_uri: Url,
    #[serde(with = "url_serde")]
    pub token_uri: Url,
}

impl BaseAuthenticator {
    pub fn new<S: Into<String>, R: AsRef<str>>(
        client_id: S,
        auth_uri: R,
        token_uri: R,
    ) -> OAuthResult<BaseAuthenticator> {
        Ok(BaseAuthenticator {
            client_id: client_id.into(),
            client_secret: None,
            auth_uri: Url::parse(auth_uri.as_ref())?,
            token_uri: Url::parse(token_uri.as_ref())?,
        })
    }

    pub fn with_secret(self, new_secret: impl Into<String>) -> BaseAuthenticator {
        BaseAuthenticator {
            client_id: self.client_id,
            client_secret: Some(new_secret.into()),
            auth_uri: self.auth_uri,
            token_uri: self.token_uri,
        }
    }

    pub fn with_no_secret(self) -> BaseAuthenticator {
        BaseAuthenticator {
            client_id: self.client_id,
            client_secret: None,
            auth_uri: self.auth_uri,
            token_uri: self.token_uri,
        }
    }
}

impl Default for BaseAuthenticator {
    fn default() -> Self {
        BaseAuthenticator {
            client_id: "foobar@example.com".into(),
            client_secret: None,
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
        match &self.client_secret {
            Some(secret) => secret.as_ref(),
            None => "",
        }
    }

    pub fn get_auth_uri(&self) -> &str {
        self.auth_uri.as_ref()
    }

    pub fn get_token_uri(&self) -> &str {
        self.token_uri.as_ref()
    }
}
