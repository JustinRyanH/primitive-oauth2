use url::Url;

use client::TokenResponse;
use client::mock_client::{MockReq, MockResp};
use client::mock_server::ServerResp;
use client::mock_server::auth_route::auth;
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

    pub fn token(&self, _: MockReq) -> ServerResp {
        MockResp::parse_access_token_response(&TokenResponse::new(
            "2YotnFZFEjr1zCsicMWpAA",
            "bearer",
        )).into()
    }

    pub fn send_request(&self, req: MockReq) -> ServerResp {
        match req.url.path() {
            "/auth" => auth(self, req),
            "/token" => self.token(req),
            _ => ServerResp::response_err(&Error::msg("404: Route not found")),
        }
    }
}
