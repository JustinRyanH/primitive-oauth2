use std::borrow::Cow;
use std::string::ToString;

use client::authenticator::BaseAuthenticator;
use client::params::UrlQueryParams;
use client::OauthClient;
use client::*;
use errors::{ErrorKind, OAuthResult};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MockClient {
    #[serde(flatten)]
    pub auth: BaseAuthenticator,
    pub scope: Vec<String>,
    pub redirect_uri: String,

    pub access_type: AccessType,
    #[serde(skip)]
    pub code: Option<String>,
    #[serde(skip)]
    pub state: Option<String>,
}

impl MockClient {
    pub fn new<S: Into<String>>(auth: BaseAuthenticator, redirect: S) -> OAuthResult<MockClient> {
        Ok(MockClient {
            auth,
            scope: vec![],
            redirect_uri: redirect.into(),
            access_type: AccessType::Grant,
            code: None,
            state: None,
        })
    }

    pub fn with_code<S: Into<String>>(self, code: S) -> MockClient {
        MockClient {
            auth: self.auth,
            scope: self.scope,
            code: Some(code.into()),
            redirect_uri: self.redirect_uri,
            access_type: self.access_type,
            state: self.state,
        }
    }

    pub fn with_scope(self, scope: Vec<String>) -> MockClient {
        MockClient {
            auth: self.auth,
            code: self.code,
            access_type: self.access_type,
            redirect_uri: self.redirect_uri,
            state: self.state,
            scope,
        }
    }

    pub fn with_state<S>(self, state: S) -> MockClient
    where
        S: Into<String>,
    {
        MockClient {
            auth: self.auth,
            code: self.code,
            access_type: self.access_type,
            redirect_uri: self.redirect_uri,
            scope: self.scope,
            state: Some(state.into()),
        }
    }

    pub fn with_no_state(self) -> MockClient {
        MockClient {
            auth: self.auth,
            code: self.code,
            access_type: self.access_type,
            redirect_uri: self.redirect_uri,
            scope: self.scope,
            state: None,
        }
    }
}

impl OauthClient for MockClient {
    type Request = MockReq;
    type Response = MockResp;

    fn get_user_auth_request<'a, 'b, S>(&'b self, storage: &'a mut S) -> OAuthResult<MockReq>
    where
        S: ClientStorage<'a, Self>,
    {
        let mut params: Vec<(&str, Cow<'b, str>)> = vec![
            ("response_type", "code".into()),
            ("client_id", self.auth.get_client_id().into()),
            ("redirect_uri", Cow::from(self.redirect_uri.as_ref())),
        ];

        let scope = self.scope
            .iter()
            .map(|v| v.as_ref())
            .collect::<Vec<&str>>()
            .join(" ");

        if !scope.is_empty() {
            params.push(("scope", scope.into()))
        }

        if let Some(ref state) = self.state {
            params.push(("state", Cow::from(state.as_ref())));
            storage.set(Cow::from(state.clone()), self.clone())?;
        }
        let mut out_url = self.auth.auth_uri.clone();
        out_url.query_pairs_mut().extend_pairs(params);

        Ok(MockReq::from(out_url))
    }

    fn handle_auth_redirect<'a, S>(
        _state_required: bool,
        req: MockReq,
        storage: &mut S,
    ) -> OAuthResult<Self>
    where
        S: ClientStorage<'a, Self>,
    {
        let params = UrlQueryParams::from(req.url.query_pairs());
        let grant_type = match params.get("grant_type") {
            // TODO: Path me
            Some(param) => AccessType::from_param_value(param)?,
            None => {
                return Err(ErrorKind::invalid_request(
                    Some("`grant_type` required for request"),
                    None,
                ))
            }
        };

        match params.get("state") {
            Some(state) => {
                let single_state = state.single().ok_or(ErrorKind::invalid_request(
                    Some("`state` must be a single parameter"),
                    None,
                ))?;
                // TODO: This should be handled by a generic AccessType
                match grant_type {
                    AccessType::Grant => {
                        let code = params
                            .get("code")
                            .ok_or(ErrorKind::invalid_request(
                                Some("`code` is required for `authorization_code` request"),
                                None,
                            ))?
                            .single()
                            .ok_or(ErrorKind::invalid_request(
                                Some("`code` must be formatted as a single request param"),
                                None,
                            ))?;

                        // TODO: this should likely be configurabled
                        return Ok(storage
                            .drop(single_state.clone().into_owned())?
                            .with_code(code.clone().into_owned()));
                    }
                    _ => return Err(ErrorKind::msg("`handle_auth_redirect` is not implemented")),
                };
            }
            None => Err(ErrorKind::msg("`handle_auth_redirect` is not implemented")),
        }
    }

    fn get_access_token_request(&self) -> OAuthResult<MockReq> {
        let mut request_url = self.auth.token_uri.clone();
        let access_type = self.access_type;
        match access_type {
            AccessType::Grant => {
                let code = self.code.clone().ok_or(ErrorKind::msg(
                    "`code` was not set for token request. It is required for explciit flow",
                ))?;
                let mut params: Vec<(&str, String)> = vec![
                    ("code", code),
                    ("grant_type", access_type.get_grant_type().into()),
                    ("client_id", self.auth.client_id.clone()),
                    ("redirect_uri", self.redirect_uri.clone()),
                ];

                request_url.query_pairs_mut().extend_pairs(params);
                Ok(MockReq::from(request_url))
            }
            AccessType::Implicit => {
                return Err(ErrorKind::msg(
                    "`get_access_token_request` implicit flow not handled",
                ));
            }
        }
    }

    fn handle_token_response<'a, S>(self, _: MockResp, _: &mut S) -> OAuthResult<Self>
    where
        S: ClientStorage<'a, Self>,
    {
        Err(ErrorKind::msg("`handle_token_response` is not implemented"))
    }
}
