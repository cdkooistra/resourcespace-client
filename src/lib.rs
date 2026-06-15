#![doc = include_str!("../README.md")]
pub mod api;
mod auth;
pub mod client;
pub use client::{Client, ClientBuilder};
pub mod error;
pub use error::Error;

static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

// TODO: tests
#[cfg(test)]
mod tests {}
