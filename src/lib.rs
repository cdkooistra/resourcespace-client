// this file should in reality only reexport the public surface

mod auth;
mod client;
pub use client::{RsClient, ClientBuilder};

mod error;
pub use error::RsError;

static APP_USER_AGENT: &str = concat!(
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
);

// TODO: tests
#[cfg(test)]
mod tests {

    // #[test]
    // fn it_works() {
    //     let result = add(2, 2);
    //     assert_eq!(result, 4);
    // }
}
