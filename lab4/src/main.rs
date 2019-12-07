use {
    num_bigint::BigUint,
    rand::Rng,
    sha2::{Digest, Sha256},
    std::string::ToString,
};

use primes::{
    generation::{generate_safe_prime, Range},
    math,
};

fn main() {
    let range = Range::new(50);

    let number = generate_safe_prime(&range);
    println!("N: {}", &number);

    let g = math::primitive_root_modulo(&number);
    println!("g: {}", g);

    let k = hash(&number, &g);
    println!("k: {}", k);
}

fn generate_string(length: usize) -> String {
    let mut rng = rand::thread_rng();

    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";

    (0..length)
        .map(|_| CHARSET[rng.gen_range(0, CHARSET.len())] as char)
        .collect()
}

fn hash<A: ToString, B: ToString>(a: &A, b: &B) -> BigUint {
    let mut hasher = Sha256::new();
    hasher.input(a.to_string());
    hasher.input(":");
    hasher.input(b.to_string());

    BigUint::from_bytes_be(hasher.result().as_slice())
}
