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

    pub fn redirect(&self, req: MockReq) -> FutResult<MockReq> {
        match self.error {
            Some(ref e) => {
                let error_kind = e.kind.clone();
                match error_kind {
                    _ => unimplemented!(),
                }
            }
            None => match req.url.path() {
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
                }
                _ => FutErr(Error::msg("404 Route not found")).pack(),
            },
        }
    }

    pub fn request(&self, req: MockReq) -> FutResult<MockResp> {
        match self.error {
            Some(_) => unimplemented!(),
            None => match req.url.path() {
                "/token" => FutOk(MockResp::from(
                    "{\"access_token\":\"2YotnFZFEjr1zCsicMWpAA\"}",
                )).pack(),
                _ => FutErr(Error::msg("404 Route not found")).pack(),
            },
        }
    }
}
