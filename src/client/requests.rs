use url::Url;

use client::params::UrlQueryParams;
use client::responses::ErrorResponse;
use errors::{Error, Result};

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
}

impl From<Url> for MockReq {
    fn from(url: Url) -> MockReq {
        MockReq {
            url,
            body: "".into(),
        }
    }
}

impl Into<UrlQueryParams> for MockReq {
    fn into(self) -> UrlQueryParams {
        UrlQueryParams::from(self.url)
    }
}
