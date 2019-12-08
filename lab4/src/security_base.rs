use {
    num_bigint::BigUint,
    rand::Rng,
    sha2::{Digest, Sha256},
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
