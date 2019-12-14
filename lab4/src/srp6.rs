use {
    num_bigint::{BigInt, BigUint, ToBigInt},
    num_traits::Zero,
    rand::Rng,
};

use primes::{ModuloGenerator, Range};

use crate::{
    security_base::{self, SecurityBase},
    server::AuthenticationData,
};

pub fn create_client_ephemeral_value<R: Rng + ?Sized>(
    security_base: &SecurityBase,
    rng: &mut R,
) -> (BigUint, BigUint) {
    let a = Range::new(EPHEMERAL_VALUE_SIZE).generate_mod(&security_base.large_prime, rng);
    let ephemeral_value = security_base.modpow(&a);

    (a, ephemeral_value)
}

pub fn create_server_ephemeral_value<R: Rng + ?Sized>(
    authentication_data: &AuthenticationData,
    security_base: &SecurityBase,
    rng: &mut R,
) -> (BigUint, BigUint) {
    let b = Range::new(EPHEMERAL_VALUE_SIZE).generate_mod(&security_base.large_prime, rng);

    let ephemeral_value =
        &security_base.k * &authentication_data.password_verifier + security_base.modpow(&b);

    (b, ephemeral_value)
}

pub fn create_client_session_key(
    username: &String,
    password: &String,
    salt: &BigUint,
    client_ephemeral_value: (&BigUint, &BigUint),
    server_ephemeral_value: &BigUint,
    security_base: &SecurityBase,
) -> (BigUint, BigUint) {
    let u = security_base::hash(&[
        client_ephemeral_value.1.to_string(),
        server_ephemeral_value.to_string(),
    ]);

    let x = security_base::hash(&[salt.to_string(), username.to_string(), password.to_string()]);

    let signed_exponent = (client_ephemeral_value.0 + &u * &x).to_bigint().unwrap();
    let signed_n = security_base.large_prime.to_bigint().unwrap();

    let signed_server_ephemeral_value = server_ephemeral_value.to_bigint().unwrap();
    let signed_subtracted_value = (&security_base.k * &security_base.modpow(&x))
        .to_bigint()
        .unwrap();

    println!("Subtracted:   {}", &signed_subtracted_value);

    let s = (signed_server_ephemeral_value - signed_subtracted_value)
        .modpow(&signed_exponent, &signed_n);

    let s = (match s {
        ref s if s.lt(&BigInt::zero()) => s + signed_n,
        s => s,
    })
    .to_biguint()
    .unwrap();

    let k = security_base::hash(&[s.to_string()]);

    (s, k)
}

pub fn create_server_session_key(
    authentication_data: &AuthenticationData,
    client_ephemeral_value: &BigUint,
    server_ephemeral_value: (&BigUint, &BigUint),
    security_base: &SecurityBase,
) -> (BigUint, BigUint) {
    let n = &security_base.large_prime;

    let u = security_base::hash(&[
        client_ephemeral_value.to_string(),
        server_ephemeral_value.1.to_string(),
    ]);

    let s = (client_ephemeral_value * &authentication_data.password_verifier.modpow(&u, n))
        .modpow(server_ephemeral_value.0, n);

    let k = security_base::hash(&[s.to_string()]);

    (s, k)
}

pub fn calculate_session_key_proof(
    username: &String,
    salt: &BigUint,
    client_ephemeral_value: &BigUint,
    server_ephemeral_value: &BigUint,
    session_key: &BigUint,
    security_base: &SecurityBase,
) -> BigUint {
    let hashed_n = security_base::hash(&[security_base.large_prime.to_string()]);
    let hashed_g = security_base::hash(&[security_base.g.to_string()]);

    let hashed_username = security_base::hash(&[username.clone()]);

    security_base::hash(&[
        (&hashed_n ^ &hashed_g).to_string(),
        hashed_username.to_string(),
        salt.to_string(),
        client_ephemeral_value.to_string(),
        server_ephemeral_value.to_string(),
        session_key.to_string(),
    ])
}

pub fn calculate_final_proof(
    client_ephemeral_value: &BigUint,
    session_key_proof: &BigUint,
    session_key_hash: &BigUint,
) -> BigUint {
    security_base::hash(&[
        client_ephemeral_value.to_string(),
        session_key_proof.to_string(),
        session_key_hash.to_string(),
    ])
}

const EPHEMERAL_VALUE_SIZE: usize = 1024;
