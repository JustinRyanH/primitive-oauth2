use url::Url;

use errors::{Error, Result};
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
}

impl MockServer {
    pub fn new() -> MockServer {
        MockServer {
            error: None,
            redirect_uri_required: false,
        }
    }

    pub fn with_error(error: MockError) -> MockServer {
        MockServer {
            error: Some(error),
            redirect_uri_required: false,
        }
    }

    pub fn require_redirect(self) -> MockServer {
        MockServer {
            error: self.error,
            redirect_uri_required: true,
        }
    }

    pub fn send_request(&self, req: MockReq) -> ServerResp {
        match req.url.path() {
            "/auth" => {
                let state = match UrlQueryParams::from(req.url.query_pairs()).get("state") {
                    Some(v) => v.single().unwrap().clone(),
                    None => {
                        return ServerResp::response_err(Error::msg(
                            "Bad Request: State should be optional, but it currently is not",
                        ));
                    }
                };
                let client_id = match UrlQueryParams::from(req.url.query_pairs()).get("client_id") {
                    Some(v) => v.single().unwrap().clone(),
                    None => return ServerResp::as_response("Bad Request: Missing `client_id`"),
                };

                if client_id != "someid@example.com" {
                    return ServerResp::as_response("Bad Request: Invalid `client_id`");
                }

                let raw_redirect_url = match UrlQueryParams::from(req.url.query_pairs())
                    .get("redirect_uri")
                {
                    Some(v) => v.single().map(|v| v.clone()),
                    None => match self.redirect_uri_required {
                        true => {
                            return ServerResp::as_response("Bad Request: Missing `redirect_uri`")
                        }
                        false => None,
                    },
                };

                let _redirect = match raw_redirect_url {
                    Some(url) => match Url::parse(url.as_ref()) {
                        Ok(u) => Some(u),
                        Err(_) => {
                            return ServerResp::as_response(
                                "Bad Request: Invalid Url for `redirect_url`",
                            )
                        }
                    },
                    None => None,
                };

                Ok(MockReq {
                    url: Url::parse_with_params(
                        "https://localhost/example/auth",
                        vec![("state", state), ("code", "MOCK_CODE".into())],
                    ).unwrap(),
                    body: String::from(""),
                }).into()
            }
            "/token" => Ok(MockResp::from(
                "{\"access_token\":\"2YotnFZFEjr1zCsicMWpAA\"}",
            )).into(),
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

impl ServerResp {
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
