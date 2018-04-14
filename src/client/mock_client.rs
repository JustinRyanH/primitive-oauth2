use url::Url;

use client::OauthClient;
use client::authenticator::BaseAuthenticator;
use client::storage::MockMemoryStorage;
use client::*;
use errors::Result;

#[derive(Debug, Clone, PartialEq)]
pub struct MockClient {
    pub auth: BaseAuthenticator,
    pub scopes: Vec<String>,
    pub redirect_uri: &'static str,
    pub access_type: AccessType,
    pub code: Option<String>,
}

impl MockClient {
    pub fn new() -> Result<MockClient> {
        Ok(MockClient {
            auth: BaseAuthenticator::new(
                "someid@example.com",
                "test",
                "http://example.com/auth",
                "http://example.com/token",
            )?,
            scopes: vec![
                "api.example.com/user.profile".to_string(),
                "api.example.com/add_item".to_string(),
            ],
            redirect_uri: "https://localhost:8080/oauth/example",
            access_type: AccessType::Grant,
            code: None,
        })
    }

    pub fn with_code<S: Into<String>>(self, code: S) -> MockClient {
        MockClient {
            auth: self.auth,
            scopes: self.scopes,
            redirect_uri: self.redirect_uri,
            access_type: self.access_type,
            code: Some(code.into()),
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
        Ok(MockReq::from(Url::parse("https://localhost")?))
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
