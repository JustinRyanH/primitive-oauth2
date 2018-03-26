use url::Url;

use errors::{Error, ErrorKind, Result};
use client::params::UrlQueryParams;
use client::mock_client::{MockReq, MockResp};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MockErrKind {
    InvalidRequest,
    UnauthorizedClient,
    AccessDenied,
    UnsupportedResponseType,
    InvalidScope,
    ServerError,
    TemporarilyUnavailable,
    Unknown,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MockError {
    pub kind: MockErrKind,
    pub description: Option<String>,
    pub url: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MockServer {
    pub error: Option<MockError>,
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

    pub fn with_error(self, error: MockError) -> MockServer {
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

    pub fn auth(&self, req: MockReq) -> ServerResp {
        let state = match MockServer::parse_state(&req.url) {
            Ok(k) => k,
            Err(e) => return ServerResp::from(e),
        };

        match MockServer::parse_client_id(&req.url) {
            Ok(v) => v,
            Err(e) => return ServerResp::from(e),
        };

        match self.parse_redirect_uri(&req.url) {
            Ok(v) => v,
            Err(e) => return ServerResp::from(e),
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

pub enum ServerResp {
    Redirect(Result<MockReq>),
    Response(Result<MockResp>),
}

impl From<Result<MockReq>> for ServerResp {
    fn from(v: Result<MockReq>) -> ServerResp {
        ServerResp::Redirect(v)
    }
}

impl From<Result<MockResp>> for ServerResp {
    fn from(v: Result<MockResp>) -> ServerResp {
        ServerResp::Response(v)
    }
}

impl From<Error> for ServerResp {
    fn from(e: Error) -> ServerResp {
        match e.kind() {
            &ErrorKind::Msg(ref s) => return ServerResp::as_response(s.as_ref()),
            &ErrorKind::InvalidRequest(ref desc, ref uri) => {
                return ServerResp::from(MockReq::from_str(format!(
                    "https://example.com?error=invalid_request&{}",
                    ServerResp::params_from_optionals(desc, uri)
                )))
            }
            &ErrorKind::UnauthorizedClient(ref desc, ref uri) => {
                return ServerResp::from(MockReq::from_str(format!(
                    "https://example.com?error=unauthorized_client&{}",
                    ServerResp::params_from_optionals(desc, uri)
                )))
            }
            _ => (),
        };
        ServerResp::response_err(e)
    }
}

impl ServerResp {
    fn params_from_optionals(description: &Option<String>, uri: &Option<String>) -> String {
        let mut out = String::from("");
        if let &Some(ref desc) = description {
            out.push_str(format!("error_description={}", desc).as_ref());
        }
        if let &Some(ref u) = uri {
            out.push_str(u.as_ref());
        }
        out
    }
    pub fn as_response<T: Into<String>>(value: T) -> ServerResp {
        ServerResp::Response(Ok(String::from(value.into()).into()))
    }

    pub fn redirect(self) -> Result<MockReq> {
        match self {
            ServerResp::Redirect(req) => req,
            _ => Err(Error::msg("Expected Redirect, but got Response")),
        }
    }

    pub fn redirect_err(err: Error) -> Self {
        ServerResp::Redirect(Err(err.into()))
    }

    pub fn response_err(err: Error) -> Self {
        ServerResp::Response(Err(err.into()))
    }

    pub fn response(self) -> Result<MockResp> {
        match self {
            ServerResp::Response(resp) => resp,
            _ => Err(Error::msg("Expected Response, but got Redirect")),
        }
    }
}
