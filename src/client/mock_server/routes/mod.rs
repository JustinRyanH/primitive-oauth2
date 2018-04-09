pub mod auth;
pub mod token;

use errors::Result;
use url::Url;
use client::mock_server::mock_server::single_param;

pub use self::auth::auth_response as auth_route;
pub use self::token::token_response as token_route;

#[cfg(test)]
mod auth_spec;
#[cfg(test)]
mod token_spec;

pub fn parse_state(url: &Url) -> Result<String> {
    single_param("state", url)
}