# Run clippy with pedantic lints
clippy:
    cargo clippy --all-targets --all-features -- -W clippy::pedantic

# Check formatting without making changes
fmt-check:
    cargo fmt --all -- --check

# Auto-fix issues with clippy and format code
autofix:
    cargo clippy --fix --all-targets --all-features --allow-dirty --allow-staged -- -W clippy::pedantic
    cargo fmt --all

# Run tests
test:
    cargo test --all-features

# Run property tests with specified number of quickcheck tests (default: 100)

proptest n="100000":
    QUICKCHECK_TESTS={{n}} cargo test --all-features prop_

# Run all CI checks (clippy, fmt-check, test)
ci: clippy fmt-check test
