# xplit

One-time pad (OTP) 2-of-2 secret sharing with CRC32 integrity checks.

Splits a secret into two shares. Both shares are required to recover the original secret. Each share alone reveals zero information (information-theoretic security).

## Usage

```bash
# Split a secret
xplit split "Hello, World!"
# Output:
# Share 1: ZiTjk3OD6puSVM/JV3CYopI=
# Share 2: LkGP/xyvysz9JqOtdpOmJ8A=

# Recover the secret
xplit recover "ZiTjk3OD6puSVM/JV3CYopI=" "LkGP/xyvysz9JqOtdpOmJ8A="
# Output: Hello, World!

# Read from stdin
echo "secret" | xplit split
```

## Features

- One-time pad encryption (information-theoretic security)
- CRC32 integrity checks
- Base64 encoding with binary data support
- CLI and web interface (WebAssembly)

## Web Interface

**[Try the live demo](https://tilacog.github.io/xplit/)** (runs entirely in your browser)

Or build and run locally (requires [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)):

```bash
just wasm-build        # Build WASM module
just wasm-dev          # Build and serve at http://localhost:8000
```

**⚠️ Demo only** - Use CLI for production/sensitive data. All computation happens in the browser.
