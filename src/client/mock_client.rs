use url::Url;

use client::OauthClient;
use client::authenticator::BaseAuthenticator;
use client::storage::MockMemoryStorage;
use client::*;
use errors::Result;

#[derive(Debug, Clone, PartialEq)]
pub struct MockClient {
    pub auth: BaseAuthenticator,
    pub scope: Vec<String>,
    pub redirect_uri: String,
    pub access_type: AccessType,
    pub code: Option<String>,
}

impl MockClient {
    pub fn new<S: Into<String>>(auth: BaseAuthenticator, redirect: S) -> Result<MockClient> {
        Ok(MockClient {
            auth,
            scope: vec![],
            redirect_uri: redirect.into(),
            access_type: AccessType::Grant,
            code: None,
        })
    }

    pub fn with_code<S: Into<String>>(self, code: S) -> MockClient {
        MockClient {
            auth: self.auth,
            scope: self.scope,
            code: Some(code.into()),
            redirect_uri: self.redirect_uri,
            access_type: self.access_type,
        }
    }

    pub fn with_scope(self, scope: Vec<String>) -> MockClient {
        MockClient {
            auth: self.auth,
            code: self.code,
            access_type: self.access_type,
            redirect_uri: self.redirect_uri,
            scope,
        }
    }
}

impl OauthClient<MockMemoryStorage> for MockClient {
    type Request = MockReq;
    type Response = MockResp;

    fn get_user_auth_request(&self, _storage: &mut MockMemoryStorage) -> Result<MockReq> {
        // let state = "EXAMPLE_STATE";
        // let mock_req = match Url::parse_with_params(
        //     self.auth.get_auth_uri(),
        //     self.auth
        //         .get_auth_params(&self.redirect_uri, &self.scopes, self.access_type, state),
        // ) {
        //     Ok(url) => Ok(MockReq {
        //         url,
        //         body: String::from(""),
        //     }),
        //     Err(e) => Err(e.into()),
        // };

        // storage
        //     .set(MockStorageKey::state(state), self.clone())
        //     .and_then(move |_| mock_req)
        Ok(MockReq::from(Url::parse_with_params(
            "https://localhost",
            vec![
                ("response_type", "code"),
                ("client_id", self.auth.get_client_id()),
                ("redirect_uri", self.redirect_uri.as_ref()),
            ],
        )?))
    }

    fn handle_auth_redirect(_req: MockReq, _storage: &mut MockMemoryStorage) -> Result<Self> {
        // let data = match ValidReq::from_url(&req.url) {
        //     Ok(d) => d,
        //     Err(e) => return Err(e.into()),
        // };
        // let code = data.code.clone();

        // match data.state {
        //     Some(state) => storage
        //         .get(MockStorageKey::State(state))
        //         .and_then(|c| Ok(c.with_code(code))),
        //     None => MockClient::new().and_then(|c| Ok(c.with_code(code))),
        // }
        unimplemented!()
    }

    fn get_access_token_request(&self) -> Result<MockReq> {
        unimplemented!()
    }

    fn handle_token_response(self, _: MockResp, _: &mut MockMemoryStorage) -> Result<Self> {
        unimplemented!()
    }
}
