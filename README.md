# Secret Share

XOR-based 2-of-2 secret sharing with CRC32 integrity checks.

## What It Does

Splits a secret into two shares. Both shares are required to recover the original secret. Each share alone reveals zero information.

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

- Information-theoretic security (one share reveals nothing)
- CRC32 checksums detect corrupted shares
- Base64 encoding for easy sharing
- Supports binary data

## How It Works

Uses XOR operation with cryptographically random data:
- Share 1 = secret ⊕ random
- Share 2 = random
- Recovery = Share 1 ⊕ Share 2
