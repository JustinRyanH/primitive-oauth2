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

pub mod errors;
pub mod client;

// SPEC ONLY CRATES
#[cfg(test)]
extern crate dotenv;
#[cfg(test)]
extern crate envy;
#[cfg(test)]
extern crate rspec;
#[cfg(test)]
extern crate futures_cpupool;
#[cfg(test)]
extern crate spectral;