use num_bigint::{BigInt, BigUint, ToBigInt, ToBigUint};

use num_traits::{One, Zero};
use primes::generation::{generate_prime, Range};

pub struct PublicKey {
    pub n: BigUint,
    pub e: BigUint,
}

pub struct PrivateKey {
    pub n: BigUint,
    pub d: BigUint,
}

pub fn encrypt(data: &BigUint, key: &PublicKey) -> BigUint {
    data.modpow(&key.e, &key.n)
}

pub fn decrypt(data: &BigUint, key: &PrivateKey) -> BigUint {
    data.modpow(&key.d, &key.n)
}

pub fn generate_keys(e: &BigUint) -> Option<(PublicKey, PrivateKey)> {
    let p = generate_prime(&Range::new(60)).to_biguint()?;
    let q = generate_prime(&Range::new(78)).to_biguint()?;

    let n = &p * &q;
    let phi = (&p - BigUint::one()) * (&q - BigUint::one());

    let d = generate_private_key(&phi, &e);

    let public_key = PublicKey {
        n: n.clone(),
        e: e.clone(),
    };

    let private_key = PrivateKey { n, d };

    println!("public key: ({}, {})", &public_key.n, &public_key.e);
    println!("private key: ({}, {})", &private_key.n, &private_key.d);

    Some((public_key, private_key))
}

fn generate_private_key(phi: &BigUint, e: &BigUint) -> BigUint {
    let phi = phi.to_bigint().unwrap();
    let e = e.to_bigint().unwrap();

    struct Row {
        a: BigInt,
        b: BigInt,
        d: BigInt,
        k: Option<BigInt>,
    };

    let mut past_row = Row {
        a: BigInt::one(),
        b: BigInt::zero(),
        d: phi.clone(),
        k: None,
    };
    let mut last_row = Row {
        a: BigInt::zero(),
        b: BigInt::one(),
        d: e.clone(),
        k: Some(&phi / &e),
    };

    loop {
        let mut current_row = Row {
            a: &past_row.a - &last_row.a * last_row.k.as_ref().unwrap(),
            b: &past_row.b - &last_row.b * last_row.k.as_ref().unwrap(),
            d: &past_row.d - &last_row.d * last_row.k.as_ref().unwrap(),
            k: None,
        };

        if current_row.d.is_one() {
            let result = match current_row.b {
                b if &b > &phi => b % phi,
                b if &b < &BigInt::zero() => b + phi,
                b => b,
            };

            return result.to_biguint().unwrap();
        }

        current_row.k = Some(&last_row.d / &current_row.d);

        past_row = last_row;
        last_row = current_row;
    }
}
