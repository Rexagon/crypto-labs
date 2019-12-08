use {num_bigint::BigUint, rand::Rng};

use primes::{ModuloGenerator, Range};

use crate::{
    client_authenticator::ClientAuthenticator,
    security_base::{self, SecurityBase},
};

pub struct Client<'a> {
    pub username: String,
    security_base: &'a SecurityBase,
}

impl<'a> Client<'a> {
    pub fn new(username: &str, security_base: &'a SecurityBase) -> Self {
        Client {
            username: username.to_string(),
            security_base,
        }
    }

    pub fn authenticate<R: Rng + ?Sized>(
        &self,
        password: &str,
        rng: &mut R,
    ) -> ClientAuthenticator {
        ClientAuthenticator::new(password, &self.security_base, rng)
    }
}
