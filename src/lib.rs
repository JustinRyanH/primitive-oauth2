//! Used for Implementation Agnostic Asynchronous
//! Oauth2 Clients
#![recursion_limit = "300"]

extern crate futures;
extern crate url;
extern crate url_serde;

#[macro_use]
extern crate error_chain;

extern crate serde;
#[macro_use]
extern crate serde_derive;

pub mod authenticator;
pub mod errors;

pub use authenticator::PrimeAuthenticator;

// SPEC ONLY CRATES
#[cfg(test)]
extern crate dotenv;
#[cfg(test)]
extern crate envy;
#[cfg(test)]
extern crate rspec;
