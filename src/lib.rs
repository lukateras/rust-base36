use num_bigint::{BigUint, ParseBigIntError};
use num_traits::Num;

const RADIX: u32 = 36;

pub fn decode(s: &str) -> Result<Vec<u8>, ParseBigIntError> {
    Ok(BigUint::from_str_radix(s, RADIX)?.to_bytes_be())
}

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
