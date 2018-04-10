use futures::future::Future;
use serde_json;
use std::sync;
use url;

error_chain! {
    errors {
        /// `InvalidRequest` for generic bad request to the Authentication Server
        ///
        /// * `human_description` - Human-Readable text providing additional
        /// information about the error, generally used to assist the
        /// client developer with additional details about the failure
        ///
        /// * `human_uri` - URI identifying a human-reable web page with the
        /// information about the error, generally used to assist the client developer
        /// with additional details about the failure
        InvalidRequest(human_description: Option<String>, human_uri: Option<String>) {
            description("The request is missing a required parameter, includes an \
            unsupported parameter value (other than grant type), \
            repeats a parameter, includes multiple credentials, \
            utilizes more than one mechanism for authenticating the \
            client, or is otherwise malformed.")
            display("Request is missing a required parameter")
        }

        /// `InvalidClient` for client authentication failures
        ///
        /// * `human_description` - Human-Readable text providing additional
        /// information about the error, generally used to assist the
        /// client developer with additional details about the failure
        ///
        /// * `human_uri` - URI identifying a human-reable web page with the
        /// information about the error, generally used to assist the client developer
        /// with additional details about the failure
        InvalidClient(human_description: Option<String>, human_uri: Option<String>) {
            description("Client authentication failed (e.g., unknown client, no \
            client authentication included, or unsupported \
            authentication method).  The authorization server MAY \
            return an HTTP 401 (Unauthorized) status code to indicate \
            which HTTP authentication schemes are supported.  If the \
            client attempted to authenticate via the \"Authorization\" \
            request header field, the authorization server MUST \
            respond with an HTTP 401 (Unauthorized) status code and \
            include the \"WWW-Authenticate\" response header field \
            matching the authentication scheme used by the client.")
            display("Client authentication failed")
        }

        /// `InvalidGrant` authorization grant or refresh token was invalid
        ///
        /// * `human_description` - Human-Readable text providing additional
        /// information about the error, generally used to assist the
        /// client developer with additional details about the failure
        ///
        /// * `human_uri` - URI identifying a human-reable web page with the
        /// information about the error, generally used to assist the client developer
        /// with additional details about the failure
        InvalidGrant(human_description: Option<String>, human_uri: Option<String>) {
            description("The provided authorization grant (e.g., authorization \
            code, resource owner credentials) or refresh token is \
            invalid, expired, revoked, does not match the redirection \
            URI used in the authorization request, or was issued to \
            another client.")
            display("Authorization Grant was Invalid")
        }

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
        UnauthorizedClient(human_description: Option<String>, human_uri: Option<String>) {
            description("The authenticated client is not authorized to use this \
            authorization grant type.")
            display("Given Client was not authorized to use given auth grant type")
        }

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
        UnsupportedGrantType(human_description: Option<String>, human_uri: Option<String>) {
            description("The authorization grant type is not supported by the \
            authorization server.")
            display("Authorization Server does not support grant type Invalid")
        }

        /// `InvalidScope` the requested scope was invalid
        ///
        /// * `human_description` - Human-Readable text providing additional
        /// information about the error, generally used to assist the
        /// client developer with additional details about the failure
        ///
        /// * `human_uri` - URI identifying a human-reable web page with the
        /// information about the error, generally used to assist the client developer
        /// with additional details about the failure
        InvalidScope(human_description: Option<String>, human_uri: Option<String>) {
            description("The requested scope is invalid, unknown, malformed, or \
            exceeds the scope granted by the resource owner.")
            display("The Given scope scope was invalid")
        }
    }

    foreign_links {
        Url(url::ParseError);
        SerdeJson(serde_json::Error);
    }
}

impl<T> From<sync::PoisonError<T>> for Error {
    fn from(v: sync::PoisonError<T>) -> Error {
        Error::from(ErrorKind::Msg(format!("SyncError: {:?}", v)))
    }
}

impl Error {
    pub fn msg<T: Into<String>>(m: T) -> Error {
        ErrorKind::Msg(m.into()).into()
    }

    #[inline]
    pub fn invalid_request<T: Into<String>>(desc: Option<T>, uri: Option<T>) -> Error {
        ErrorKind::InvalidRequest(desc.map(|v| v.into()), uri.map(|v| v.into())).into()
    }
    pub fn invalid_client<T: Into<String>>(desc: Option<T>, uri: Option<T>) -> Error {
        ErrorKind::InvalidClient(desc.map(|v| v.into()), uri.map(|v| v.into())).into()
    }
    pub fn invalid_grant<T: Into<String>>(desc: Option<T>, uri: Option<T>) -> Error {
        ErrorKind::InvalidGrant(desc.map(|v| v.into()), uri.map(|v| v.into())).into()
    }
    pub fn unauthorized_client<T: Into<String>>(desc: Option<T>, uri: Option<T>) -> Error {
        ErrorKind::UnauthorizedClient(desc.map(|v| v.into()), uri.map(|v| v.into())).into()
    }
    pub fn unsupported_grant_type<T: Into<String>>(desc: Option<T>, uri: Option<T>) -> Error {
        ErrorKind::UnsupportedGrantType(desc.map(|v| v.into()), uri.map(|v| v.into())).into()
    }
    pub fn invalid_scope<T: Into<String>>(desc: Option<T>, uri: Option<T>) -> Error {
        ErrorKind::InvalidScope(desc.map(|v| v.into()), uri.map(|v| v.into())).into()
    }
}

impl PartialEq for Error {
    fn eq(&self, other: &Error) -> bool {
        self.kind() == other.kind()
    }
}

impl PartialEq for ErrorKind {
    fn eq(&self, other: &ErrorKind) -> bool {
        match (self, other) {
            (&ErrorKind::Msg(ref l), &ErrorKind::Msg(ref r)) => l == r,
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
