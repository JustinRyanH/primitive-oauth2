use errors::{Error, ErrorKind};

/// [4.2.2.  Access Token Response](https://tools.ietf.org/html/rfc6749#section-4.2.2)
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub token_type: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_in: Option<usize>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

impl TokenResponse {
    pub fn new<T: Into<String>, S: Into<String>>(access_token: T, token_type: S) -> TokenResponse {
        TokenResponse {
            access_token: access_token.into(),
            token_type: token_type.into(),
            expires_in: None,
            scope: None,
            state: None,
        }
    }

    pub fn with_state<T: Into<String>>(self, state: T) -> TokenResponse {
        TokenResponse {
            access_token: self.access_token,
            token_type: self.token_type,
            expires_in: self.expires_in,
            scope: self.scope,
            state: Some(state.into()),
        }
    }

    pub fn with_scope<T: Clone + Into<String>>(self, scope: &Vec<T>) -> TokenResponse {
        TokenResponse {
            access_token: self.access_token,
            token_type: self.token_type,
            expires_in: self.expires_in,
            scope: Some(scope.into_iter().map(|v| v.clone().into()).collect()),
            state: self.state,
        }
    }

    pub fn with_expiration(self, expiration: usize) -> TokenResponse {
        TokenResponse {
            access_token: self.access_token,
            token_type: self.token_type,
            expires_in: Some(expiration),
            scope: self.scope,
            state: self.state,
        }
    }
}

/// [4.2.2.1.  Error Response](https://tools.ietf.org/html/rfc6749#section-4.2.2.1)
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: &'static str,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_uri: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

impl ErrorResponse {
    #[inline]
    pub fn with_state(self, state: String) -> ErrorResponse {
        ErrorResponse {
            error: self.error,
            error_description: self.error_description,
            error_uri: self.error_uri,
            state: Some(state),
        }
    }
}

impl<'a> From<&'a ErrorKind> for ErrorResponse {
    #[inline]
    fn from(kind: &'a ErrorKind) -> ErrorResponse {
        let (error, error_description, error_uri): (
            &'static str,
            Option<String>,
            Option<String>,
        ) = match kind {
            ErrorKind::Msg(msg) => ("server_error", Some(msg.clone()), None),
            ErrorKind::InvalidRequest(desc, uri) => ("invalid_request", desc.clone(), uri.clone()),
            ErrorKind::InvalidClient(desc, uri) => ("invalid_client", desc.clone(), uri.clone()),
            ErrorKind::InvalidGrant(desc, uri) => ("invalid_grant", desc.clone(), uri.clone()),
            ErrorKind::UnauthorizedClient(desc, uri) => {
                ("unauthorized_client", desc.clone(), uri.clone())
            }
            ErrorKind::UnsupportedGrantType(desc, uri) => {
                ("unsupported_grant_type", desc.clone(), uri.clone())
            }
            ErrorKind::InvalidScope(desc, uri) => ("invalid_scope", desc.clone(), uri.clone()),
            _ => (
                "unknown_error",
                Some("Failed to Recongize Given ErrorKind".to_string()),
                None,
            ),
        };
        ErrorResponse {
            error,
            error_description: error_description,
            error_uri: error_uri,
            state: None,
        }
    }
}

impl<'a> From<&'a Error> for ErrorResponse {
    #[inline]
    fn from(e: &'a Error) -> ErrorResponse {
        e.kind().into()
    }
}

impl IntoIterator for ErrorResponse {
    type Item = (&'static str, String);
    type IntoIter = ::std::vec::IntoIter<(&'static str, String)>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        let mut out = vec![("error", self.error.into())];

        match self.error_description {
            Some(desc) => out.push(("error_description", desc)),
            None => (),
        };
        match self.error_uri {
            Some(uri) => out.push(("error_uri", uri)),
            None => (),
        };
        match self.state {
            Some(state) => out.push(("state", state)),
            None => (),
        };
        out.into_iter()
    }
}
