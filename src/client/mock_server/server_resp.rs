use client::{MockReq, MockResp};
use errors::{ErrorKind, OAuthResult};

pub enum ServerResp {
    Redirect(OAuthResult<MockReq>),
    Response(OAuthResult<MockResp>),
}

impl From<OAuthResult<MockReq>> for ServerResp {
    fn from(v: OAuthResult<MockReq>) -> ServerResp {
        ServerResp::Redirect(v)
    }
}

impl From<OAuthResult<MockResp>> for ServerResp {
    fn from(v: OAuthResult<MockResp>) -> ServerResp {
        ServerResp::Response(v)
    }
}

impl ServerResp {
    pub fn as_response<T: Into<String>>(value: T) -> ServerResp {
        ServerResp::Response(Ok(String::from(value.into()).into()))
    }

    pub fn redirect(self) -> OAuthResult<MockReq> {
        match self {
            ServerResp::Redirect(req) => req,
            _ => Err(ErrorKind::msg("Expected Redirect, but got Response")),
        }
    }

    pub fn redirect_err(err: &ErrorKind) -> Self {
        ServerResp::Redirect(MockReq::parse_error_req("https://example.com", &err))
    }

    pub fn response_err(err: &ErrorKind) -> Self {
        ServerResp::Response(MockResp::parse_error_resp(&err))
    }

    pub fn response(self) -> OAuthResult<MockResp> {
        match self {
            ServerResp::Response(resp) => resp,
            _ => Err(ErrorKind::msg("Expected Response, but got Redirect")),
        }
    }
}
