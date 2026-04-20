# Contributing

Thanks a lot for your interest in contributing to `resourcespace-client`.

## Development Setup

You will need:

- Rust stable (via [rustup](https://rustup.rs))
- A ResourceSpace instance for manual integration testing (optional)

Clone the repo and build:

```sh
git clone https://github.com/cdkooistra/resourcespace-client
cd resourcespace-client
cargo build
```

## Running Checks

```sh
cargo clippy --all-targets --all-features -- -D warnings # lint
cargo fmt --check   # formatting
```

Or using the justfile:

```sh
just fmt
just clippy
```

## Commit Convention

This project uses [Conventional Commits](https://www.conventionalcommits.org), and thus commit messages must follow the format (e.g.: `feat: new endpoint`, `docs: update readme`, `chore: bump dependencies`).

## Pull Requests

Consider opening an issue first to discuss the approach.

- Keep PRs focused
- Update `CHANGELOG.md`
- Ensure all checks pass

## Design Principles

- Follow the existing patterns (builder methods, `impl Into<T>` parameters, and `List<T>` for comma-separated value fields)
- Verify endpoint parameters against the [RS knowledge base](https://www.resourcespace.com/knowledge-base/api/) before implementing
