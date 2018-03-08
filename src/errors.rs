error_chain! {
    types {
        OauthError, OauthErrorKind, Result;
    }

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
            display("The request is missing a required parameter")
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
            description(" Client authentication failed (e.g., unknown client, no \
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

    }
}
