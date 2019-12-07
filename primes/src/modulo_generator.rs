use {
    num_bigint::BigUint,
    rand::{distributions::uniform::UniformSampler, prelude::Rng},
};

use crate::range::Range;

pub trait ModuloGenerator {
    fn generate_mod<R: Rng + ?Sized>(&self, modulo: &BigUint, rng: &mut R) -> BigUint;
}

impl ModuloGenerator for Range {
    fn generate_mod<R: Rng + ?Sized>(&self, modulo: &BigUint, rng: &mut R) -> BigUint {
        self.uniform.sample(rng) % modulo
    }
}
