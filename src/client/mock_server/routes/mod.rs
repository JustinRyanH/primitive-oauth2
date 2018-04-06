pub mod auth;
pub mod token;

pub use self::auth::auth_response as auth_route;
pub use self::token::token_response as token_route;

#[cfg(test)]
mod auth_spec;
#[cfg(test)]
mod token_spec;
