pub mod auth;
pub mod token;

use std::borrow::Cow;

use client::params::UrlQueryParams;
use errors::{Error, Result};
use url::Url;

pub use self::auth::auth_response as auth_route;
pub use self::token::token_response as token_route;

#[cfg(test)]
mod auth_spec;
#[cfg(test)]
mod token_spec;

#[inline]
pub fn parse_state<'a>(url: &'a Url) -> Result<Cow<'a, str>> {
    single_param("state", url)
}

#[inline]
pub fn maybe_single_param<'a>(name: &'static str, url: &'a Url) -> Option<Cow<'a, str>> {
    match UrlQueryParams::from(url.query_pairs()).get(name) {
        Some(v) => v.single().map(|v| v.clone()),
        None => None,
    }
}

#[inline]
pub fn single_param<'a>(name: &'static str, url: &'a Url) -> Result<Cow<'a, str>> {
    match UrlQueryParams::from(url).get(name) {
        Some(v) => Ok(v.single()
            .ok_or(Error::msg(
                "Bad Request: Expected Single Parameter, found many",
            ))?
            .clone()),
        None => Err(Error::invalid_request(
            Some(format!("Bad Request: Missing `{}`", name)),
            None,
        )),
    }
}
