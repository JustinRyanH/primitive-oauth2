mod server_resp;
#[cfg(test)]
mod spec;

use url::Url;

use errors::{Error, ErrorKind, Result};
use client::params::UrlQueryParams;
use client::mock_client::{MockReq, MockResp};

use self::server_resp::ServerResp;

const VALID_SCOPES: [&'static str; 2] =
    ["api.example.com/user.profile", "api.example.com/add_item"];

#[derive(Debug, PartialEq)]
pub struct MockServer {
    pub error: Option<ErrorKind>,
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

    pub fn with_error(self, error: ErrorKind) -> MockServer {
        MockServer {
            error: Some(error),
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
        match UrlQueryParams::from(url.query_pairs()).get("state") {
            Some(v) => Ok(v.single()
                .ok_or(Error::msg(
                    "Bad Request: Expected Single Parameter, found many",
                ))?
                .clone()),
            None => Err(Error::invalid_request(
                Some("Bad Request: Missing `state`"),
                None,
            )),
        }
    }

    pub fn parse_client_id(url: &Url) -> Result<String> {
        let client_id = match UrlQueryParams::from(url.query_pairs()).get("client_id") {
            Some(v) => v.single()
                .ok_or(Error::msg(
                    "Bad Request: Expected Single Parameter, found many",
                ))?
                .clone(),
            None => {
                return Err(Error::invalid_request(
                    Some("Bad Request: Missing `client_id`"),
                    None,
                ))
            }
        };

        if client_id != "someid@example.com" {
            return Err(Error::unauthorized_client(
                Some("Unauthorized: Client Not Authorized"),
                None,
            ));
        }
        Ok(client_id)
    }

    pub fn parse_redirect_uri(&self, url: &Url) -> Result<Option<Url>> {
        let raw_redirect_url = match UrlQueryParams::from(url.query_pairs()).get("redirect_uri") {
            Some(v) => v.single().map(|v| v.clone()),
            None => match self.redirect_uri_required {
                true => {
                    return Err(Error::invalid_request(
                        Some("Bad Request: Missing `redirect_uri`"),
                        None,
                    ))
                }
                false => None,
            },
        };

        match raw_redirect_url {
            Some(url) => {
                if url.as_str() != "https://localhost:8080/oauth/example" {
                    return Err(Error::invalid_request(
                        Some("Bad Request: Redirect Uri does not match valid uri"),
                        None,
                    ));
                }
                match Url::parse(url.as_ref()) {
                    Ok(u) => Ok(Some(u)),
                    _ => unreachable!(),
                }
            }
            None => Ok(None),
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
        let state = match MockServer::parse_state(&req.url) {
            Ok(k) => k,
            Err(e) => return ServerResp::redirect_err(e),
        };

        match MockServer::parse_client_id(&req.url) {
            Ok(_) => (),
            Err(e) => return ServerResp::redirect_err(e),
        };

        match self.parse_redirect_uri(&req.url) {
            Ok(_) => (),
            Err(e) => return ServerResp::redirect_err(e),
        };

        match self.parse_scope(&req.url) {
            Ok(_) => (),
            Err(e) => return ServerResp::redirect_err(e),
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
            _ => ServerResp::response_err(Error::msg("404: Route not found")),
        }
    }
}
