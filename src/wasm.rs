//! WASM bindings for xplit
//!
//! This module provides JavaScript-friendly bindings for the core split/recover functionality.

use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use crate::{recover_secret, split_secret};

/// Initialize panic hook for better error messages in the browser console
#[wasm_bindgen(start)]
pub fn init() {
    console_error_panic_hook::set_once();
}

/// Use wee_alloc as the global allocator for smaller WASM binary size
#[cfg(target_arch = "wasm32")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// Result of a split operation (for JSON serialization)
#[derive(Serialize, Deserialize)]
pub struct SplitResult {
    /// The first share (base64 encoded)
    pub share1: String,
    /// The second share (base64 encoded)
    pub share2: String,
}

/// Split a secret into two XOR-based shares with CRC32 integrity checks
///
/// # Arguments
/// * `secret` - The secret text to split
///
/// # Returns
/// JSON string containing both shares (base64 encoded), or an error message
///
/// # Example (JavaScript)
/// ```javascript
/// const result = wasm_split("my secret message");
/// const data = JSON.parse(result);
/// console.log(`Share 1: ${data.share1}`);
/// console.log(`Share 2: ${data.share2}`);
/// ```
#[wasm_bindgen]
pub fn wasm_split(secret: &str) -> Result<String, JsValue> {
    // Validate input
    if secret.is_empty() {
        return Err(JsValue::from_str("Secret cannot be empty"));
    }

    let secret_bytes = secret.as_bytes();

    // Perform the split
    let shares = split_secret(secret_bytes)
        .map_err(|e| JsValue::from_str(&format!("Split failed: {}", e)))?;

    // Encode shares as base64
    let result = SplitResult {
        share1: BASE64.encode(&shares.share1),
        share2: BASE64.encode(&shares.share2),
    };

    // Serialize to JSON
    serde_json::to_string(&result)
        .map_err(|e| JsValue::from_str(&format!("Serialization failed: {}", e)))
}

/// Recover the original secret from two shares
///
/// # Arguments
/// * `share1` - First share (base64 encoded)
/// * `share2` - Second share (base64 encoded)
///
/// # Returns
/// The recovered secret as a string, or an error message
///
/// # Example (JavaScript)
/// ```javascript
/// const share1 = "ZiTjk3OD6puSVM/JV3CYopI=";
/// const share2 = "LkGP/xyvysz9JqOtdpOmJ8A=";
/// const secret = wasm_recover(share1, share2);
/// console.log(`Recovered secret: ${secret}`);
/// ```
#[wasm_bindgen]
pub fn wasm_recover(share1: &str, share2: &str) -> Result<String, JsValue> {
    // Decode from base64
    let share1_bytes = BASE64
        .decode(share1)
        .map_err(|e| JsValue::from_str(&format!("Failed to decode share1: {}", e)))?;

    let share2_bytes = BASE64
        .decode(share2)
        .map_err(|e| JsValue::from_str(&format!("Failed to decode share2: {}", e)))?;

    // Perform the recovery
    let recovered = recover_secret(&share1_bytes, &share2_bytes)
        .map_err(|e| JsValue::from_str(&format!("Recovery failed: {}", e)))?;

    // Convert to UTF-8 string
    String::from_utf8(recovered)
        .map_err(|e| JsValue::from_str(&format!("Recovered data is not valid UTF-8: {}", e)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wasm_split_basic() {
        let secret = "Hello, World!";
        let result = wasm_split(secret);
        assert!(result.is_ok());

        let json = result.unwrap();
        let data: SplitResult = serde_json::from_str(&json).unwrap();

        // Shares should be base64 encoded
        assert!(!data.share1.is_empty());
        assert!(!data.share2.is_empty());

        // Should be able to decode the shares
        assert!(BASE64.decode(&data.share1).is_ok());
        assert!(BASE64.decode(&data.share2).is_ok());
    }

    #[test]
    fn test_wasm_split_empty() {
        let result = wasm_split("");
        assert!(result.is_err());
    }

    #[test]
    fn test_wasm_recover_readme_example() {
        let share1 = "ZiTjk3OD6puSVM/JV3CYopI=";
        let share2 = "LkGP/xyvysz9JqOtdpOmJ8A=";

        let result = wasm_recover(share1, share2);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Hello, World!");
    }

    #[test]
    fn test_wasm_split_and_recover() {
        let secret = "Test secret message";

        // Split
        let split_result = wasm_split(secret).unwrap();
        let data: SplitResult = serde_json::from_str(&split_result).unwrap();

        // Recover
        let recovered = wasm_recover(&data.share1, &data.share2);
        assert!(recovered.is_ok());
        assert_eq!(recovered.unwrap(), secret);
    }

    #[test]
    fn test_wasm_recover_invalid_base64() {
        let result = wasm_recover("not valid base64!!!", "also not valid!!!");
        assert!(result.is_err());
    }

    #[test]
    fn test_wasm_recover_corrupted_share() {
        // Valid base64 but corrupted share (wrong checksum)
        let result = wasm_recover("AAAAAAAAAAAAAA==", "BBBBBBBBBBBBBB==");
        assert!(result.is_err());
    }
}
