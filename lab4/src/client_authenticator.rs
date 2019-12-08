use {num_bigint::BigUint, rand::Rng};

use primes::{ModuloGenerator, Range};

use crate::security_base::{self, SecurityBase};

pub struct ClientAuthenticator<'a> {
    password: String,
    a: BigUint,
    ephemeral_value: BigUint,
    session_key: Option<BigUint>,

    security_base: &'a SecurityBase,
}

impl<'a> ClientAuthenticator<'a> {
    pub fn new<R: Rng + ?Sized>(
        password: &str,
        security_base: &'a SecurityBase,
        rng: &mut R,
    ) -> Self {
        let a = Range::new(1024).generate_mod(&security_base.large_prime, rng);

        let ephemeral_value = security_base.modpow(&a);

        ClientAuthenticator {
            password: password.to_string(),
            a,
            ephemeral_value,
            session_key: None,
            security_base,
        }
    }
}
