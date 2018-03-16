use url_serde;
use url::Url;

use client::AccessType;

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
        access_type: AccessType,
        state: &str,
    ) -> Vec<(&str, String)> {
        let parsed_scopes: Vec<(&str, String)> = scopes
            .into_iter()
            .map(|scope| ("scope", scope.clone()))
            .collect();
        vec![
            ("client_id", self.client_id.clone()),
            ("redirect_uri", String::from(redirect_uri)),
            ("response_type", access_type.get_response_type().to_string()),
            ("state", state.into()),
        ].into_iter()
            .chain(parsed_scopes)
            .collect()
    }
}
