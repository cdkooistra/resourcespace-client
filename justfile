fmt:
    @cargo fmt

fmt-check:
    @cargo fmt --check

clippy:
    @cargo clippy --all-targets --all-features -- -D warnings
