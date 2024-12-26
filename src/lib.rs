//! Base36 encoder/decoder.

use num_bigint::{BigUint, ParseBigIntError};
use num_traits::Num;

const RADIX: u32 = 36;

/// Error during `decode`.
pub type DecodeError = ParseBigIntError;

/// Decodes a Base36 string into a byte array.
///
/// Returns `DecodeError` if the string is not Base36.
pub fn decode(s: &str) -> Result<Vec<u8>, DecodeError> {
    Ok(BigUint::from_str_radix(s, RADIX)?.to_bytes_be())
}

/// Encodes a byte array into a Base36 string (lowercase).
pub fn encode(buf: &[u8]) -> String {
    BigUint::from_bytes_be(buf).to_str_radix(RADIX)
}

#[cfg(test)]
mod tests {
    use super::*;

    const HELLO_WORLD: &[u8] = b"Hello, World!";
    const HELLO_WORLD_BASE36: &str = "fg3h7vqw7een6jwwnzmp";
    const HELLO_WORLD_BASE36_MIXED: &str = "fG3h7vQw7eEn6JwwnZmP";

    #[test]
    fn test_decode() {
        assert_eq!(decode(&HELLO_WORLD_BASE36).unwrap(), HELLO_WORLD);
    }

    #[test]
    fn test_decode_mixed() {
        assert_eq!(decode(&HELLO_WORLD_BASE36_MIXED).unwrap(), HELLO_WORLD);
    }

    #[test]
    fn test_decode_empty() {
        assert!(decode("").is_err());
    }

    #[test]
    fn test_decode_nul() {
        assert_eq!(decode("0").unwrap(), b"\0");
    }

    #[test]
    fn test_encode() {
        assert_eq!(encode(HELLO_WORLD), HELLO_WORLD_BASE36);
    }

    #[test]
    fn test_encode_empty() {
        assert_eq!(encode(b""), "0");
    }

    #[test]
    fn test_encode_nul() {
        assert_eq!(encode(b"\0"), "0");
    }
}
