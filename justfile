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

# Build WASM module for web (requires wasm-pack)
wasm-build:
    wasm-pack build --target web --out-dir docs/pkg

# Serve the web demo locally for testing
wasm-serve:
    @echo "Serving on http://localhost:8000"
    @echo "Press Ctrl+C to stop"
    python3 -m http.server 8000 --directory docs

# Build WASM and serve locally
wasm-dev: wasm-build wasm-serve

# Clean WASM build artifacts
wasm-clean:
    rm -rf docs/pkg target/wasm32-unknown-unknown

# Check that WASM builds without errors
wasm-check:
    cargo check --lib --target wasm32-unknown-unknown --no-default-features
