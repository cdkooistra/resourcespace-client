# resourcespace-client

A Rust client library for the [ResourceSpace](https://www.resourcespace.com/) Digital Asset Management API.

[![GitHub](https://img.shields.io/badge/github-resourcespace--client-blue?logo=github&label=cdkooistra)](https://github.com/cdkooistra/resourcespace-client) [![crates.io](https://img.shields.io/badge/crates.io-resourcespace--client-orange?logo=rust)](https://crates.io/crates/resourcespace-client) [![docs.rs](https://img.shields.io/badge/docs.rs-resourcespace--client-red?logo=rust)](https://docs.rs/resourcespace-client)

`resourcespace-client` provides an ergonomic async Rust interface to the ResourceSpace API. ResourceSpace is an open-source Digital Asset Management system developed.

- Support userkey and sessionkey authentication
- Support all (non-native usermode) API endpoints
- Async API
- Create resources, metadata fields, and more via builder patterns.

## Quick start

This example initializes a simple client to search for resources and then update a metadata field. For more examples view the [examples directory](./examples/).

```rust,no_run
use resourcespace_client::Client;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = Client::builder()
        .base_url("https://your-rs-instance.com/")?
        .user_key("your_username", "your_api_key")
        .build()?;

    // Search for resources
    let results = client
        .search()
        .do_search(DoSearchRequest::new("landscape"))
        .await?;

    // Update a metadata field
    client
        .metadata()
        .update_field(UpdateFieldRequest::new(42, "title", "My Asset"))
        .await?;

    Ok(())
}
```

## Status

This crate follows [Semantic Versioning](https://semver.org). While on `0.x`, minor versions may contain breaking changes as the API stabilises. See [CHANGELOG.md](CHANGELOG.md) for details.

## License

Licensed under either of

- [Apache License, Version 2.0](LICENSE-APACHE)
- [MIT license](LICENSE-MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
