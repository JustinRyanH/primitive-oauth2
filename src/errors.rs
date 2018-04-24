use std::{error::Error, fmt};

use futures::future::Future;
use serde_json;
use std::sync;
use url;

pub type OAuthResult<T> = Result<T, ErrorKind>;

#[derive(Debug, Clone)]
pub enum ErrorKind {
    /// `InvalidRequest` for generic bad request to the Authentication Server
    ///
    /// * `human_description` - Human-Readable text providing additional
    /// information about the error, generally used to assist the
    /// client developer with additional details about the failure
    ///
    /// * `human_uri` - URI identifying a human-reable web page with the
    /// information about the error, generally used to assist the client developer
    /// with additional details about the failure
    InvalidRequest(Option<String>, Option<String>),

    /// `InvalidClient` for client authentication failures
    ///
    /// * `human_description` - Human-Readable text providing additional
    /// information about the error, generally used to assist the
    /// client developer with additional details about the failure
    ///
    /// * `human_uri` - URI identifying a human-reable web page with the
    /// information about the error, generally used to assist the client developer
    /// with additional details about the failure
    InvalidClient(Option<String>, Option<String>),

    /// `InvalidGrant` authorization grant or refresh token was invalid
    ///
    /// * `human_description` - Human-Readable text providing additional
    /// information about the error, generally used to assist the
    /// client developer with additional details about the failure
    ///
    /// * `human_uri` - URI identifying a human-reable web page with the
    /// information about the error, generally used to assist the client developer
    /// with additional details about the failure
    InvalidGrant(Option<String>, Option<String>),

    /// `UnauthorizedClient` When the client was not authorized to use
    ///  given auth grant type
    ///
    /// * `human_description` - Human-Readable text providing additional
    /// information about the error, generally used to assist the
    /// client developer with additional details about the failure
    ///
    /// * `human_uri` - URI identifying a human-reable web page with the
    /// information about the error, generally used to assist the client developer
    /// with additional details about the failure
    UnauthorizedClient(Option<String>, Option<String>),

    /// `UnsupportedGrantType` authorization server does not support grant
    /// type
    ///
    /// * `human_description` - Human-Readable text providing additional
    /// information about the error, generally used to assist the
    /// client developer with additional details about the failure
    ///
    /// * `human_uri` - URI identifying a human-reable web page with the
    /// information about the error, generally used to assist the client developer
    /// with additional details about the failure
    UnsupportedGrantType(Option<String>, Option<String>),

    /// `InvalidScope` the requested scope was invalid
    ///
    /// * `human_description` - Human-Readable text providing additional
    /// information about the error, generally used to assist the
    /// client developer with additional details about the failure
    ///
    /// * `human_uri` - URI identifying a human-reable web page with the
    /// information about the error, generally used to assist the client developer
    /// with additional details about the failure
    InvalidScope(Option<String>, Option<String>),

    /// `ParseError` error parsing a Url into ErrorKind
    ParseError(url::ParseError),
    /// `
    /// Error was not important enough to handle with it's own
    /// kind
    UnknownError(String),
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ErrorKind::InvalidRequest(ref desc, ref uri) => write!(
                f,
                "Request is missing a required parameter: Desc({:?}), Uri({:?})",
                desc, uri
            ),
            ErrorKind::InvalidClient(ref desc, ref uri) => write!(
                f,
                "Client authentication failed: Desc({:?}), Uri({:?})",
                desc, uri
            ),
            ErrorKind::InvalidGrant(ref desc, ref uri) => write!(
                f,
                "Authorization Grant was Invalid: Desc({:?}), Uri({:?})",
                desc, uri
            ),
            ErrorKind::UnauthorizedClient(ref desc, ref uri) => write!(
                f,
                "Given Client was not authorized to use given auth grant type: Desc({:?}), Uri({:?})",
                desc, uri
            ),
            ErrorKind::UnsupportedGrantType(ref desc, ref uri) => write!(
                f,
                "Authorization Server does not support grant type Invalid: Desc({:?}), Uri({:?})",
                desc, uri
            ),
            ErrorKind::InvalidScope(ref desc, ref uri) => write!(
                f,
                "The Given scope scope was invalid: Desc({:?}), Uri({:?})",
                desc, uri
            ),
            ErrorKind::ParseError(ref error) => write!(f, "ParseError: {}", error),
            ErrorKind::UnknownError(ref error) => write!(f, "{:?}", error),
        }
    }
}

impl Error for ErrorKind {
    fn description(&self) -> &str {
        match *self {
            ErrorKind::InvalidRequest(_, _) => {
                "The request is missing a required parameter, includes an \
                 unsupported parameter value (other than grant type), \
                 repeats a parameter, includes multiple credentials, \
                 utilizes more than one mechanism for authenticating the \
                 client, or is otherwise malformed."
            }
            ErrorKind::InvalidClient(_, _) => {
                "Client authentication failed (e.g., unknown client, no \
                 client authentication included, or unsupported \
                 authentication method).  The authorization server MAY \
                 return an HTTP 401 (Unauthorized) status code to indicate \
                 which HTTP authentication schemes are supported.  If the \
                 client attempted to authenticate via the \"Authorization\" \
                 request header field, the authorization server MUST \
                 respond with an HTTP 401 (Unauthorized) status code and \
                 include the \"WWW-Authenticate\" response header field \
                 matching the authentication scheme used by the client."
            }
            ErrorKind::InvalidGrant(_, _) => {
                "The provided authorization grant (e.g., authorization \
                 code, resource owner credentials) or refresh token is \
                 invalid, expired, revoked, does not match the redirection \
                 URI used in the authorization request, or was issued to \
                 another client."
            }
            ErrorKind::UnauthorizedClient(_, _) => {
                "The authenticated client is not authorized to use this \
                 authorization grant type."
            }
            ErrorKind::UnsupportedGrantType(_, _) => {
                "The authorization grant type is not supported by the \
                 authorization server."
            }
            ErrorKind::InvalidScope(_, _) => {
                "The requested scope is invalid, unknown, malformed, or \
                 exceeds the scope granted by the resource owner."
            }
            ErrorKind::ParseError(_) => "Errors parsing the URI",
            ErrorKind::UnknownError(_) => "Errors too lazy to specify",
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            ErrorKind::ParseError(ref error) => error.cause(),
            _ => None,
        }
    }
}

impl<T> From<sync::PoisonError<T>> for ErrorKind {
    fn from(v: sync::PoisonError<T>) -> ErrorKind {
        ErrorKind::UnknownError(format!("SyncError: {:?}", v))
    }
}

impl From<url::ParseError> for ErrorKind {
    fn from(v: url::ParseError) -> ErrorKind {
        ErrorKind::ParseError(v)
    }
}

impl<'a> From<&'a str> for ErrorKind {
    fn from(v: &'a str) -> ErrorKind {
        ErrorKind::UnknownError(v.into())
    }
}

impl From<serde_json::Error> for ErrorKind {
    fn from(v: serde_json::Error) -> ErrorKind {
        ErrorKind::UnknownError(format!("SerdeJsonError: {:?}", v))
    }
}

impl ErrorKind {
    pub fn msg<T: Into<String>>(m: T) -> ErrorKind {
        ErrorKind::UnknownError(m.into())
    }

    #[inline]
    pub fn invalid_request<T: Into<String>>(desc: Option<T>, uri: Option<T>) -> ErrorKind {
        ErrorKind::InvalidRequest(desc.map(|v| v.into()), uri.map(|v| v.into()))
    }
    pub fn invalid_client<T: Into<String>>(desc: Option<T>, uri: Option<T>) -> ErrorKind {
        ErrorKind::InvalidClient(desc.map(|v| v.into()), uri.map(|v| v.into()))
    }
    pub fn invalid_grant<T: Into<String>>(desc: Option<T>, uri: Option<T>) -> ErrorKind {
        ErrorKind::InvalidGrant(desc.map(|v| v.into()), uri.map(|v| v.into()))
    }
    pub fn unauthorized_client<T: Into<String>>(desc: Option<T>, uri: Option<T>) -> ErrorKind {
        ErrorKind::UnauthorizedClient(desc.map(|v| v.into()), uri.map(|v| v.into()))
    }
    pub fn unsupported_grant_type<T: Into<String>>(desc: Option<T>, uri: Option<T>) -> ErrorKind {
        ErrorKind::UnsupportedGrantType(desc.map(|v| v.into()), uri.map(|v| v.into()))
    }
    pub fn invalid_scope<T: Into<String>>(desc: Option<T>, uri: Option<T>) -> ErrorKind {
        ErrorKind::InvalidScope(desc.map(|v| v.into()), uri.map(|v| v.into()))
    }
}

impl PartialEq for ErrorKind {
    fn eq(&self, other: &ErrorKind) -> bool {
        match (self, other) {
            (&ErrorKind::UnknownError(ref l), &ErrorKind::UnknownError(ref r)) => l == r,
            (
                &ErrorKind::InvalidRequest(ref this_desc, ref this_uri),
                &ErrorKind::InvalidRequest(ref other_desc, ref other_uri),
            ) => this_desc == other_desc && this_uri == other_uri,
            (
                &ErrorKind::InvalidClient(ref this_desc, ref this_uri),
                &ErrorKind::InvalidClient(ref other_desc, ref other_uri),
            ) => this_desc == other_desc && this_uri == other_uri,
            (
                &ErrorKind::InvalidGrant(ref this_desc, ref this_uri),
                &ErrorKind::InvalidGrant(ref other_desc, ref other_uri),
            ) => this_desc == other_desc && this_uri == other_uri,
            (
                &ErrorKind::UnauthorizedClient(ref this_desc, ref this_uri),
                &ErrorKind::UnauthorizedClient(ref other_desc, ref other_uri),
            ) => this_desc == other_desc && this_uri == other_uri,
            (
                &ErrorKind::UnsupportedGrantType(ref this_desc, ref this_uri),
                &ErrorKind::UnsupportedGrantType(ref other_desc, ref other_uri),
            ) => this_desc == other_desc && this_uri == other_uri,
            (
                &ErrorKind::InvalidScope(ref this_desc, ref this_uri),
                &ErrorKind::InvalidScope(ref other_desc, ref other_uri),
            ) => this_desc == other_desc && this_uri == other_uri,
            _ => false,
        }
    }
}

pub type FutResult<T> = Box<Future<Item = T, Error = Error> + Send>;
