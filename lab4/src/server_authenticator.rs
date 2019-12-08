use {num_bigint::BigUint, rand::Rng};

use primes::{ModuloGenerator, Range};

use crate::{
    security_base::{self, SecurityBase},
    server::UserData,
};

pub struct ServerAuthenticator<'a, 'b> {
    b: BigUint,
    ephemeral_value: BigUint,
    session_key: Option<BigUint>,

    data: &'a UserData,
    security_base: &'b SecurityBase,
}

impl<'a, 'b> ServerAuthenticator<'a, 'b> {
    pub fn new<R: Rng + ?Sized>(
        data: &'a UserData,
        security_base: &'b SecurityBase,
        rng: &mut R,
    ) -> Self {
        let b = Range::new(1024).generate_mod(&security_base.large_prime, rng);

        let ephemeral_value = (&security_base.k * &data.password_verifier
            + security_base.modpow(&b))
            % &security_base.large_prime;

        ServerAuthenticator {
            b,
            ephemeral_value,
            session_key: None,

            data,
            security_base,
        }
    }

    pub fn calculate_session_key(&mut self, other: &BigUint) {
        let n = &self.security_base.large_prime;

        let scrambling_parameter =
            security_base::hash(&[other.to_string(), self.ephemeral_value.to_string()]);

        let s = (other * &self.data.password_verifier.modpow(&scrambling_parameter, n))
            .modpow(&self.b, n);

        let session_key = security_base::hash(&[s.to_string()]);

        println!("Session key: {}", &session_key);

        self.session_key = Some(session_key);
    }
}
