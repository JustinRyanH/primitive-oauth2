use futures::future::{err as FutErr, ok as FutOk};
use url::Url;

use errors::Error;
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
                FutOk(MockReq {
                    url: Url::parse_with_params(
                        "https://localhost/example/auth",
                        vec![("state", state), ("code", "MOCK_CODE".into())],
                    ).unwrap(),
                    body: String::from(""),
                }).pack()
                    .into()
            }
            "/token" => FutOk(MockResp::from(
                "{\"access_token\":\"2YotnFZFEjr1zCsicMWpAA\"}",
            )).pack()
                .into(),
            _ => ServerResp::response_err(Error::msg("404 Route not found")),
        }
    }
}

pub enum ServerResp {
    Redirect(FutResult<MockReq>),
    Response(FutResult<MockResp>),
}

impl From<FutResult<MockReq>> for ServerResp {
    fn from(v: FutResult<MockReq>) -> ServerResp {
        ServerResp::Redirect(v)
    }
}

impl From<FutResult<MockResp>> for ServerResp {
    fn from(v: FutResult<MockResp>) -> ServerResp {
        ServerResp::Response(v)
    }
}

impl ServerResp {
    pub fn redirect(self) -> Option<FutResult<MockReq>> {
        match self {
            ServerResp::Redirect(req) => Some(req),
            _ => None,
        }
    }

    pub fn redirect_err(err: Error) -> Self {
        ServerResp::Redirect(FutErr(err.into()).pack())
    }

    pub fn response_err(err: Error) -> Self {
        ServerResp::Response(FutErr(err.into()).pack())
    }

    pub fn response(self) -> Option<FutResult<MockResp>> {
        match self {
            ServerResp::Response(resp) => Some(resp),
            _ => None,
        }
    }
}
