use url_serde;
use url::Url;
use errors::Result;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BaseAuthenticator {
    client_id: String,
    client_secret: String,
    #[serde(with = "url_serde")]
    auth_uri: Url,
    #[serde(with = "url_serde")]
    token_uri: Url,
}

impl BaseAuthenticator {
    pub fn new<S: Into<String>, R: AsRef<str>>(client_id: S, client_secret: S, auth_uri: R, token_uri: R) -> Result<BaseAuthenticator> {
        Ok(BaseAuthenticator {
            client_id: client_id.into(),
            client_secret: client_secret.into(),
            auth_uri: Url::parse(auth_uri.as_ref())?,
            token_uri: Url::parse(token_uri.as_ref())?,
        })
    }
}

impl Default for BaseAuthenticator {
    fn default() -> Self {
        BaseAuthenticator {
            client_id: "foobar@example.com".into(),
            client_secret: "foobar_secret".into(),
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
}
