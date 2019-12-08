use {num_bigint::BigUint, rand::Rng, std::collections::HashMap};

use primes::{ModuloGenerator, Range};

use crate::{
    security_base::{self, SecurityBase},
    server_authenticator::ServerAuthenticator,
};

pub struct Server<'a> {
    users: HashMap<String, UserData>,
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
            security_base::hash(&[username.to_string(), password.to_string(), salt.to_string()]);

        let password_verifier = self.security_base.modpow(&x);

        self.users.insert(
            username.to_string(),
            UserData {
                username: username.to_string(),
                salt,
                password_verifier,
            },
        );
    }

    pub fn authenticate<R: Rng + ?Sized>(
        &self,
        username: &str,
        rng: &mut R,
    ) -> Option<ServerAuthenticator> {
        let user = self.users.get(&username.to_string());

        user.map(|data| ServerAuthenticator::new(data, &self.security_base, rng))
    }
}

pub struct UserData {
    pub username: String,
    pub salt: BigUint,
    pub password_verifier: BigUint,
}
