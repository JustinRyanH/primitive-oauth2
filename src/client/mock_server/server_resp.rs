use errors::{Error, ErrorKind, Result};
use client::mock_client::{MockReq, MockResp};

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
    fn params_from_optionals(description: &Option<String>, uri: &Option<String>) -> String {
        let mut out = String::from("");
        if let &Some(ref desc) = description {
            out.push_str(format!("error_description={}", desc).as_ref());
        }
        if let &Some(ref u) = uri {
            out.push_str(format!("error_uri={}", u).as_ref());
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

    pub fn redirect_err(err: &Error) -> Self {
        match err.kind() {
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
            &ErrorKind::InvalidGrant(ref desc, ref uri) => {
                return ServerResp::from(MockReq::from_str(format!(
                    "https://example.com?error=invalid_client&{}",
                    ServerResp::params_from_optionals(desc, uri)
                )))
            }
            &ErrorKind::InvalidClient(ref desc, ref uri) => {
                return ServerResp::from(MockReq::from_str(format!(
                    "https://example.com?error=invalid_client&{}",
                    ServerResp::params_from_optionals(desc, uri)
                )))
            }
            &ErrorKind::UnsupportedGrantType(ref desc, ref uri) => {
                return ServerResp::from(MockReq::from_str(format!(
                    "https://example.com?error=unsupported_grant_type&{}",
                    ServerResp::params_from_optionals(desc, uri)
                )))
            }
            &ErrorKind::InvalidScope(ref desc, ref uri) => {
                return ServerResp::from(MockReq::from_str(format!(
                    "https://example.com?error=invalid_scope&{}",
                    ServerResp::params_from_optionals(desc, uri)
                )))
            }
            _ => (),
        };
        ServerResp::Response(Ok(MockResp::from(err.kind())))
    }

    pub fn response_err(err: Error) -> Self {
        ServerResp::Response(Ok(MockResp::from(err.kind())))
    }

    pub fn response(self) -> Result<MockResp> {
        match self {
            ServerResp::Response(resp) => resp,
            _ => Err(Error::msg("Expected Response, but got Redirect")),
        }
    }
}
