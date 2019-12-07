use {
    num_bigint::{BigUint, UniformBigUint},
    num_traits::{One, Zero},
    rand::distributions::uniform::UniformSampler,
};

pub struct Range {
    pub bit_count: usize,
    pub uniform: UniformBigUint,
}

impl Range {
    pub fn new(bit_count: usize) -> Self {
        Range {
            bit_count,
            uniform: UniformBigUint::new(BigUint::zero(), BigUint::one() << bit_count),
        }
    }
}
