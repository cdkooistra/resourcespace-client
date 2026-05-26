fmt:
    @cargo fmt

fmt-check:
    @cargo fmt --check

clippy:
    @cargo clippy --all-targets --all-features -- -D warnings

docs:
    @cargo doc --no-deps
    @echo "Rendering docs at http://localhost:8080/resourcespace_client/"
    @cd target/doc && python3 -m http.server 8080
