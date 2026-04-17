#![doc = include_str!("../README.md")]
mod auth;
pub mod api;
pub mod client;
pub mod error;

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
