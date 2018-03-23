use futures::IntoFuture;
use futures::future::{err as FutErr, ok as FutOk};
use url::Url;

use errors::{Error, Result};
use client::params::{ParamValue, UrlQueryParams};
use client::mock_client::{MockReq, MockResp};
use client::{AsyncPacker, FutResult};

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
}

impl MockServer {
    pub fn new() -> MockServer {
        MockServer { error: None }
    }

    pub fn with_error(error: MockError) -> MockServer {
        MockServer { error: Some(error) }
    }

    pub fn send_request(&self, req: MockReq) -> ServerResp {
        match req.url.path() {
            "/auth" => {
                let state = match UrlQueryParams::from(req.url.query_pairs())
                    .get("state")
                    .unwrap_or(ParamValue::from(""))
                    .single()
                    {
                        Some(v) => v.clone(),
                        None => String::from(""),
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
            _ => ServerResp::response_err(Error::msg("404 Route not found")),
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
