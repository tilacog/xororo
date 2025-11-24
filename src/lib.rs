use crc32fast::Hasher;
use rand::RngCore;
use rand::rng;
use thiserror::Error;

/// Split a secret into exactly 2 shares - both required for recovery
#[derive(Debug)]
pub struct TwoShares {
    pub share1: Vec<u8>, // secret ⊕ rand + crc32
    pub share2: Vec<u8>, // rand + crc32
}

/// Error type for share validation
#[derive(Debug, Error, PartialEq)]
pub enum ShareError {
    #[error("Invalid checksum - share data may be corrupted")]
    InvalidChecksum,
    #[error("Share is too short to contain valid data")]
    ShareTooShort,
    #[error("Input is empty - cannot process empty secrets or shares")]
    EmptyInput,
}

/// Split secret into 2 shares with CRC32 checksums
///
/// # Errors
///
/// Returns [`ShareError::EmptyInput`] if the secret is empty.
pub fn split_secret(secret: &[u8]) -> Result<TwoShares, ShareError> {
    if secret.is_empty() {
        return Err(ShareError::EmptyInput);
    }

    let mut share2_data = vec![0u8; secret.len()];
    let mut rand_gen = rng();
    rand_gen.fill_bytes(&mut share2_data); // Generate random data

    // share1 is secret XOR'd with the random data
    let share1_data: Vec<u8> = secret
        .iter()
        .zip(share2_data.iter())
        .map(|(s, r)| s ^ r)
        .collect();

    // Calculate CRC32 for each share
    let mut hasher1 = Hasher::new();
    hasher1.update(&share1_data);
    let crc1 = hasher1.finalize();

    let mut hasher2 = Hasher::new();
    hasher2.update(&share2_data);
    let crc2 = hasher2.finalize();

    // Append CRC32 to each share (4 bytes, big-endian)
    let mut share1 = share1_data;
    share1.extend_from_slice(&crc1.to_be_bytes());

    let mut share2 = share2_data;
    share2.extend_from_slice(&crc2.to_be_bytes());

    Ok(TwoShares { share1, share2 })
}

/// Verify CRC32 checksum and extract data
fn verify_and_extract(share: &[u8]) -> Result<Vec<u8>, ShareError> {
    if share.is_empty() {
        return Err(ShareError::EmptyInput);
    }

    if share.len() < 4 {
        return Err(ShareError::ShareTooShort);
    }

    let data_len = share.len() - 4;
    let data = &share[..data_len];
    let stored_crc = u32::from_be_bytes([
        share[data_len],
        share[data_len + 1],
        share[data_len + 2],
        share[data_len + 3],
    ]);

    let mut hasher = Hasher::new();
    hasher.update(data);
    let computed_crc = hasher.finalize();

    if computed_crc != stored_crc {
        return Err(ShareError::InvalidChecksum);
    }

    Ok(data.to_vec())
}

/// Recover secret from both shares, verifying checksums
///
/// # Errors
///
/// Returns:
/// - [`ShareError::EmptyInput`] if either share is empty
/// - [`ShareError::ShareTooShort`] if either share is shorter than 4 bytes
/// - [`ShareError::InvalidChecksum`] if either share has a corrupted checksum
pub fn recover_secret(share1: &[u8], share2: &[u8]) -> Result<Vec<u8>, ShareError> {
    let data1 = verify_and_extract(share1)?;
    let data2 = verify_and_extract(share2)?;

    Ok(data1
        .iter()
        .zip(data2.iter())
        .map(|(s1, s2)| s1 ^ s2)
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};

    #[test]
    fn test_readme_example() {
        // Example from README with specific shares
        let share1_b64 = "ZiTjk3OD6puSVM/JV3CYopI=";
        let share2_b64 = "LkGP/xyvysz9JqOtdpOmJ8A=";

        let share1 = BASE64.decode(share1_b64).expect("valid base64 for share1");
        let share2 = BASE64.decode(share2_b64).expect("valid base64 for share2");

        let recovered = recover_secret(&share1, &share2).expect("recovery should succeed");
        let recovered_str = String::from_utf8(recovered).expect("valid UTF-8");

        assert_eq!(recovered_str, "Hello, World!");
    }

    quickcheck::quickcheck! {
        fn prop_split_and_recover(secret: Vec<u8>) -> bool {
            if secret.is_empty() {
                // Empty secrets should return EmptyInput error
                return matches!(split_secret(&secret), Err(ShareError::EmptyInput));
            }

            let shares = split_secret(&secret).expect("split should succeed for non-empty input");
            let recovered = recover_secret(&shares.share1, &shares.share2);
            recovered.ok() == Some(secret)
        }
    }
}
