extern crate futures;
extern crate url;

extern crate serde;
#[macro_use]
extern crate serde_derive;

mod authenticator;

pub use authenticator::PrimeAuthenticator;

// SPEC ONLY CRATES
#[cfg(test)]
extern crate dotenv;
#[cfg(test)]
extern crate envy;
#[cfg(test)]
extern crate rspec;
