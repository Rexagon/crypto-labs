use {
    num_bigint::BigUint,
    rand::Rng,
    sha2::{Digest, Sha256},
    std::{convert::TryFrom, string::ToString},
};

use primes::{math, PrimeGenerator, Range};

#[derive(Clone)]
pub struct SecurityBase {
    pub large_prime: BigUint,
    pub g: BigUint,
    pub k: BigUint,
}

impl SecurityBase {
    pub fn new<R: Rng + ?Sized>(range: &Range, rng: &mut R) -> Self {
        let large_prime = range.generate_safe_prime(rng);
        let g = math::primitive_root_modulo(&large_prime);
        let k = hash(&[large_prime.to_string(), g.to_string()]);

        println!("{}\n{}\n", large_prime, g);

        SecurityBase { large_prime, g, k }
    }

    pub fn modpow(&self, value: &BigUint) -> BigUint {
        self.g.modpow(value, &self.large_prime)
    }
}

pub fn hash(args: &[String]) -> BigUint {
    let mut hasher = Sha256::new();

    hasher.input(args.join(":"));

    BigUint::from_bytes_be(hasher.result().as_slice())
}

impl TryFrom<&String> for SecurityBase {
    type Error = ParseError;

    fn try_from(string: &String) -> Result<Self, Self::Error> {
        if string.is_empty() {
            return Err(ParseError::EmptyArgument);
        }

        let parts = string.split('/').collect::<Vec<_>>();
        if parts.len() != 2 {
            return Err(ParseError::InvalidFormat);
        }

        let (large_prime_data, g_data) = (parts[0], parts[1]);

        // Try parse large prime
        let large_prime = match base64::decode_config(large_prime_data, base64::URL_SAFE) {
            Ok(data) => data,
            Err(error) => return Err(ParseError::DecodeError(error)),
        };
        let large_prime = BigUint::from_bytes_be(large_prime.as_slice());

        // Try parse g
        let g = match base64::decode_config(g_data, base64::URL_SAFE) {
            Ok(data) => data,
            Err(error) => return Err(ParseError::DecodeError(error)),
        };
        let g = BigUint::from_bytes_be(g.as_slice());

        let k = hash(&[large_prime.to_string(), g.to_string()]);

        Ok(SecurityBase { large_prime, g, k })
    }
}

#[derive(Debug)]
pub enum ParseError {
    EmptyArgument,
    InvalidFormat,
    DecodeError(base64::DecodeError),
}

impl ToString for SecurityBase {
    fn to_string(&self) -> String {
        let large_prime_data = self.large_prime.to_bytes_be();
        let g_data = self.g.to_bytes_be();

        format!(
            "{}/{}",
            base64::encode_config(&large_prime_data, base64::URL_SAFE),
            base64::encode_config(&g_data, base64::URL_SAFE)
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const LARGE_PRIME: &[u8] =
        b"12659713704398683678252721740794913973831179758540205633708419793371328559433\
        861279378764130623543390389748518257576579274673062550550577434500021009201747";
    const G: u64 = 2;
    const ENCODED: &str = "8bdkP5bqH0037QuzcoeW_pw1WvGYlIW3L7y33ECiK1-huL\
                           O6SbHfh68Ps6pT25vnvgziD6Y_v-tEwAl-eUs6Uw==/Ag==";

    #[test]
    fn test_conversion_to_string() {
        let large_prime = BigUint::parse_bytes(LARGE_PRIME, 10).unwrap();
        let g = BigUint::from(G);
        let k = hash(&[large_prime.to_string(), g.to_string()]);

        let security_base = SecurityBase { large_prime, g, k };

        assert_eq!(security_base.to_string(), ENCODED);
    }

    #[test]
    fn test_conversion_from_string() {
        let large_prime = BigUint::parse_bytes(LARGE_PRIME, 10).unwrap();
        let g = BigUint::from(G);
        let k = hash(&[large_prime.to_string(), g.to_string()]);

        let security_base = SecurityBase::try_from(&String::from(ENCODED)).unwrap();

        assert_eq!(security_base.large_prime, large_prime);
        assert_eq!(security_base.g, g);
        assert_eq!(security_base.k, k);
    }
}
