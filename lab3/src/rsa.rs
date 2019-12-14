use {
    num_bigint::{BigInt, BigUint, ToBigInt},
    num_traits::{One, Zero},
    rand::Rng,
};

use primes::{PrimeGenerator, Range};

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

pub fn generate_keys<R: Rng + ?Sized>(e: &BigUint, rng: &mut R) -> (PublicKey, PrivateKey) {
    let p = Range::new(1024).generate_prime(rng);
    let q = Range::new(1024).generate_prime(rng);

    let n = &p * &q;
    let phi = (&p - BigUint::one()) * (&q - BigUint::one());

    let d = generate_private_key(&phi, &e);

    let public_key = PublicKey {
        n: n.clone(),
        e: e.clone(),
    };

    let private_key = PrivateKey { n, d };

    (public_key, private_key)
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
                ref b if b > &phi => b % &phi,
                ref b if b < &BigInt::zero() => b + phi,
                b => b,
            };

            return result.to_biguint().unwrap();
        }

        current_row.k = Some(&last_row.d / &current_row.d);

        past_row = last_row;
        last_row = current_row;
    }
}
