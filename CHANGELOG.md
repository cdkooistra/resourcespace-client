# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-04-20

### Added

- Initial implementation of `ResourceApi`, `CollectionApi`, `MetadataApi`,
  `UserApi`, `SearchApi`, `SystemApi`, and `MessageApi`
- `Client` with typestate builder supporting user key and session key authentication
- `List<T>` type for ergonomic CSV parameter construction
- `FieldIdentifier` for accepting field references as either numeric ID or shortname
- `FieldValue` for typed metadata field values, automatically handling `nodevalues`
- `FetchRows` for typed search pagination
