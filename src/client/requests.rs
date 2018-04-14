use std::borrow::Cow;
use url::Url;

use client::params::UrlQueryParams;
use client::responses::ErrorResponse;
use errors::{Error, Result};

#[derive(Debug, PartialEq, Clone)]
pub struct ValidReq<'a> {
    pub code: Cow<'a, str>,
    pub state: Option<Cow<'a, str>>,
}

impl<'a> ValidReq<'a> {
    pub fn from_url<T: Into<UrlQueryParams<'a>> + Clone>(into_params: &T) -> Result<ValidReq<'a>> {
        let params: UrlQueryParams = into_params.clone().into();
        let code: Cow<'a, str> = params
            .get("code")
            .ok_or("Requires a code to authorize token")?
            .single()
            .ok_or("Expected the code to be a single value")?
            .clone();
        let state = match params.get("state") {
            Some(n) => n.single().cloned(),
            None => None,
        };

        Ok(ValidReq { code, state })
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct MockReq {
    pub url: Url,
    pub body: String,
}

impl MockReq {
    // TODO: Repalce with FromStr trait
    pub fn from_str<T: AsRef<str>>(s: T) -> Result<MockReq> {
        Ok(Url::parse(s.as_ref())?.into())
    }

    pub fn parse_error_req(url: &'static str, err: &Error) -> Result<MockReq> {
        Ok(Url::parse_with_params(url, ErrorResponse::from(err).into_iter())?.into())
    }

    pub fn is_empty(&self) -> bool {
        self.url.clone().query_pairs().count() == 0
    }
}

impl From<Url> for MockReq {
    fn from(url: Url) -> MockReq {
        MockReq {
            url,
            body: "".into(),
        }
    }
}

