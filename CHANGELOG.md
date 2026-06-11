# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2026-06-11

### Added

- CI for running cargo (doc)tests

### Changed

- Overhauled error handling to a more Rust idiomatic way
- Renamed `RsError` to `Error` (breaking!!)
- Correct return value for `send_request_multipart`
- Changed the way the library handles bool as u8 (now more ergonomically)

### Fixed

- Missing builder methods
- Fragile URL assembly
- Fallible `ClientBuilder.base_url()`
- Incorrect types for `SearchGetPreviewsRequest`
- Non-compiling doctests

## [0.1.2] - 2026-06-04

### Added

- .zed folder for developing in Zed editor

### Changed

- Configurable timeout and user agent on `ClientBuilder`
- Moved DRY function for building reqwest::Client into `ClientBuilder`

## [0.1.1] - 2026-05-26

### Added

- Streaming support for `upload_multipart()`.
- just recipes: `just docs`, `just (default)`

### Changed

- Some internal refactoring.
- Bumped devenv version.

## [0.1.0] - 2026-04-20

### Added

- Initial implementation of `ResourceApi`, `CollectionApi`, `MetadataApi`,
  `UserApi`, `SearchApi`, `SystemApi`, and `MessageApi`
- `Client` with typestate builder supporting user key and session key authentication
- `List<T>` type for ergonomic CSV parameter construction
- `FieldIdentifier` for accepting field references as either numeric ID or shortname
- `FieldValue` for typed metadata field values, automatically handling `nodevalues`
- `FetchRows` for typed search pagination
