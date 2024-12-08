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

    #[test]
    fn test_decode() {
        assert_eq!(decode("fg3h7vqw7een6jwwnzmp").unwrap(), b"Hello, World!");
    }

    #[test]
    fn test_encode() {
        assert_eq!(encode(b"Hello, World!"), "fg3h7vqw7een6jwwnzmp");
    }
}
