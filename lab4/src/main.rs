use {
    num_bigint::BigUint,
    rand::{Rng, ThreadRng},
    sha2::{Digest, Sha256},
    std::string::ToString,
};

use primes::{math, PrimeGenerator, Range};

fn main() {
    let mut rng = rand::thread_rng();
    let range = Range::new(50);

    let field = SafetyField::new(&range, &mut rng);
}

fn generate_string(length: usize, rng: &mut ThreadRng) -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";

    (0..length)
        .map(|_| CHARSET[rng.gen_range(0, CHARSET.len())] as char)
        .collect()
}

/// Safety field of H, N, g, k
struct SafetyField {
    large_prime: BigUint,
    g: BigUint,
    k: BigUint,
}

impl SafetyField {
    fn new<R: Rng + ?Sized>(range: &Range, rng: &mut R) -> Self {
        let large_prime = range.generate_safe_prime(rng);
        let g = math::primitive_root_modulo(&large_prime);
        let k = Self::hash(&large_prime, &g);

        SafetyField { large_prime, g, k }
    }

    fn hash<A: ToString + ?Sized, B: ToString + ?Sized>(a: &A, b: &B) -> BigUint {
        let mut hasher = Sha256::new();
        hasher.input(a.to_string());
        hasher.input(":");
        hasher.input(b.to_string());

        BigUint::from_bytes_be(hasher.result().as_slice())
    }
}
