use {num_bigint::BigUint, rand::Rng, std::collections::HashMap};

use primes::{ModuloGenerator, Range};

use crate::security_base::{self, SecurityBase};

pub struct Server<'a> {
    users: HashMap<String, AuthenticationData>,
    security_base: &'a SecurityBase,
}

impl<'a> Server<'a> {
    pub fn new(security_base: &'a SecurityBase) -> Self {
        Server {
            users: HashMap::new(),
            security_base,
        }
    }

    pub fn register<R: Rng + ?Sized>(&mut self, username: &str, password: &str, rng: &mut R) {
        let salt = Range::new(64).generate_mod(&self.security_base.large_prime, rng);
        let x =
            security_base::hash(&[salt.to_string(), username.to_string(), password.to_string()]);

        let password_verifier = self.security_base.modpow(&x);

        self.users.insert(
            username.to_string(),
            AuthenticationData {
                username: username.to_string(),
                salt,
                password_verifier,
            },
        );
    }

    pub fn find(&self, username: &String) -> Option<&AuthenticationData> {
        self.users.get(username)
    }
}

pub struct AuthenticationData {
    pub username: String,
    pub salt: BigUint,
    pub password_verifier: BigUint,
}
