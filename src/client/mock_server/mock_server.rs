use url::Url;

use client::mock_client::{MockReq, MockResp};
use client::mock_server::{ServerResp, VALID_SCOPES};
use client::params::UrlQueryParams;
use errors::{Error, Result};

#[inline]
pub fn validate_required_uri(url: String) -> Result<Option<Url>> {
    if url.as_str() != "https://localhost:8080/oauth/example" {
        return Err(Error::invalid_request(
            Some("Bad Request: Redirect Uri does not match valid uri"),
            None,
        ));
    }
    match Url::parse(url.as_ref()) {
        Ok(u) => Ok(Some(u)),
        Err(e) => Err(e.into()),
    }
}

#[inline]
pub fn maybe_single_param(name: &'static str, url: &Url) -> Option<String> {
    match UrlQueryParams::from(url.query_pairs()).get(name) {
        Some(v) => v.single().map(|v| v.clone()),
        None => None,
    }
}

#[inline]
pub fn single_param(name: &'static str, url: &Url) -> Result<String> {
    match UrlQueryParams::from(url.query_pairs()).get(name) {
        Some(v) => Ok(v.single()
            .ok_or(Error::msg(
                "Bad Request: Expected Single Parameter, found many",
            ))?
            .clone()),
        None => Err(Error::invalid_request(
            Some(format!("Bad Request: Missing `{}`", name)),
            None,
        )),
    }
}

#[derive(Debug, PartialEq)]
pub struct MockServer {
    pub error: Option<Error>,
    pub redirect_uri_required: bool,
    pub code: &'static str,
    pub last_state: Option<&'static str>,
}

impl MockServer {
    pub fn new() -> MockServer {
        MockServer {
            error: None,
            redirect_uri_required: false,
            code: "",
            last_state: None,
        }
    }

    pub fn with_error<T>(self, error: T) -> MockServer
    where
        T: Into<Error>,
    {
        MockServer {
            error: Some(error.into()),
            code: self.code,
            redirect_uri_required: self.redirect_uri_required,
            last_state: self.last_state,
        }
    }

    pub fn require_redirect(self) -> MockServer {
        MockServer {
            error: self.error,
            code: self.code,
            redirect_uri_required: true,
            last_state: self.last_state,
        }
    }

    pub fn with_code(self, code: &'static str) -> MockServer {
        MockServer {
            error: self.error,
            code: code,
            redirect_uri_required: self.redirect_uri_required,
            last_state: self.last_state,
        }
    }

    pub fn with_state(self, state: &'static str) -> MockServer {
        MockServer {
            error: self.error,
            code: self.code,
            redirect_uri_required: self.redirect_uri_required,
            last_state: Some(state),
        }
    }

    pub fn with_no_state(self) -> MockServer {
        MockServer {
            error: self.error,
            code: self.code,
            redirect_uri_required: self.redirect_uri_required,
            last_state: None,
        }
    }

    pub fn parse_state(url: &Url) -> Result<String> {
        single_param("state", url)
    }

    pub fn parse_client_id(url: &Url) -> Result<String> {
        let client_id = single_param("client_id", url)?;

        if client_id != "someid@example.com" {
            return Err(Error::unauthorized_client(
                Some("Unauthorized: Client Not Authorized"),
                None,
            ));
        }
        Ok(client_id)
    }

    pub fn parse_redirect_uri(&self, url: &Url) -> Result<Option<Url>> {
        match maybe_single_param("redirect_uri", url) {
            Some(v) => Ok(validate_required_uri(v)?),
            None => match self.redirect_uri_required {
                true => {
                    return Err(Error::invalid_request(
                        Some("Bad Request: Missing `redirect_uri`"),
                        None,
                    ));
                }
                false => Ok(None),
            },
        }
    }

    pub fn parse_scope(&self, url: &Url) -> Result<()> {
        let scope: Vec<String> = match UrlQueryParams::from(url.query_pairs()).get("scope") {
            Some(v) => v.into_iter().collect(),
            None => vec![],
        };

        for value in scope {
            if !VALID_SCOPES.into_iter().any(|&v| v == value) {
                return Err(Error::invalid_request(
                    None,
                    Some(format!(
                        "https://docs.example.com/scopes?invalid_scope={}",
                        value
                    )),
                ));
            }
        }

        Ok(())
    }

    pub fn auth(&self, req: MockReq) -> ServerResp {
        if let Some(ref err) = self.error {
            return ServerResp::redirect_err(err);
        }
        let state = match MockServer::parse_state(&req.url) {
            Ok(k) => k,
            Err(e) => return ServerResp::redirect_err(&e),
        };

        match MockServer::parse_client_id(&req.url) {
            Ok(_) => (),
            Err(e) => return ServerResp::redirect_err(&e),
        };

        match self.parse_redirect_uri(&req.url) {
            Ok(_) => (),
            Err(e) => return ServerResp::redirect_err(&e),
        };

        match self.parse_scope(&req.url) {
            Ok(_) => (),
            Err(e) => return ServerResp::redirect_err(&e),
        };

        Ok(MockReq {
            url: Url::parse_with_params(
                "https://localhost/example/auth",
                vec![("state", state), ("code", "MOCK_CODE".into())],
            ).unwrap(),
            body: String::from(""),
        }).into()
    }

    pub fn token(&self, _: MockReq) -> ServerResp {
        Ok(MockResp::from(
            "{\"access_token\":\"2YotnFZFEjr1zCsicMWpAA\"}",
        )).into()
    }

    pub fn send_request(&self, req: MockReq) -> ServerResp {
        match req.url.path() {
            "/auth" => self.auth(req),
            "/token" => self.token(req),
            _ => ServerResp::response_err(&Error::msg("404: Route not found")),
        }
    }
}
