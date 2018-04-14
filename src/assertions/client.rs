use spectral::{AssertionFailure, Spec};
use client::{AccessType, mock_client::MockClient};

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
