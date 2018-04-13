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
                Url::parse("http://example.com/auth")?,
                Url::parse("http://example.com/token")?,
            ),
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

#[cfg(test)]
pub mod test_helpers {
    use super::*;
    use spectral::{AssertionFailure, Spec};

    pub trait MockClientHelper<'s> {
        fn has_code(&mut self) -> Spec<'s, String>;
        fn has_no_code(&mut self);
        fn has_access_type_of(&mut self, expected_type: AccessType);
        fn has_redirect_uri_of(&mut self, expected_uri: &'static str);
        fn has_scopes_of<'a, T: Clone + Into<String>>(&mut self, expected_scope: &'a Vec<T>);
    }

    impl<'s> MockClientHelper<'s> for Spec<'s, MockClient> {
        fn has_code(&mut self) -> Spec<'s, String> {
            match self.subject.code {
                Some(ref val) => Spec {
                    subject: val,
                    subject_name: self.subject_name,
                    location: self.location.clone(),
                    description: self.description,
                },
                None => {
                    AssertionFailure::from_spec(self)
                        .with_expected(format!("`MockClient.code` with Some(String)"))
                        .with_actual(format!("`MockClient.code` is None"))
                        .fail();
                    unreachable!();
                }
            }
        }

        fn has_no_code(&mut self) {
            match self.subject.code {
                None => (),
                Some(ref val) => {
                    AssertionFailure::from_spec(self)
                        .with_expected(format!("`MockClient.code` to be None"))
                        .with_actual(format!("`MockClient.code` is option<{:?}>", val))
                        .fail();
                    unreachable!();
                }
            }
        }

        fn has_access_type_of(&mut self, expected_type: AccessType) {
            let subject_type = self.subject.access_type;

            if subject_type == expected_type {
                ()
            } else {
                AssertionFailure::from_spec(self)
                    .with_expected(format!("`MockClient.access_type` of {:?}", expected_type))
                    .with_actual(format!("`MockClient.access_type` of {:?}", subject_type))
                    .fail();
                unreachable!();
            }
        }

        fn has_redirect_uri_of(&mut self, expected_uri: &'static str) {
            let subject_uri = self.subject.redirect_uri;
            if subject_uri == expected_uri {
                ()
            } else {
                AssertionFailure::from_spec(self)
                    .with_expected(format!("`MockClient.redirect_uri` of {:?}", expected_uri))
                    .with_actual(format!("`MockClient.redirect_uri` of {:?}", subject_uri))
                    .fail();
                unreachable!();
            }
        }

        fn has_scopes_of<'a, T: Clone + Into<String>>(&mut self, expected_scopes: &'a Vec<T>) {
            let subject_scopes = &self.subject.scopes;
            let ref parsed_scopes: Vec<String> = expected_scopes
                .into_iter()
                .map(|v| v.clone().into())
                .collect();

            if subject_scopes == parsed_scopes {
                ()
            } else {
                AssertionFailure::from_spec(self)
                    .with_expected(format!("`MockClient.scopes` of {:?}", parsed_scopes))
                    .with_actual(format!("`MockClient.scopes` of {:?}", subject_scopes))
                    .fail();
                unreachable!();
            }
        }
    }
}
