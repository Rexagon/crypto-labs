use {
    num_bigint::{BigUint, UniformBigUint},
    num_traits::{One, Zero},
    rand::{distributions::uniform::UniformSampler, prelude::ThreadRng},
};

use crate::prime;

pub struct Range {
    bit_count: usize,
    uniform: UniformBigUint,
}

impl Range {
    pub fn new(bit_count: usize) -> Self {
        Range {
            bit_count,
            uniform: UniformBigUint::new(BigUint::zero(), BigUint::one() << bit_count),
        }
    }
}

pub fn generate_prime(range: &Range) -> BigUint {
    let mut rng = rand::thread_rng();

    loop {
        if let Some(number) = try_generate_prime(range, &mut rng) {
            return number;
        }
    }
}

pub fn generate_safe_prime(range: &Range) -> BigUint {
    let mut rng = rand::thread_rng();

    loop {
        if let Some(number) = try_generate_safe_prime(range, &mut rng) {
            return number;
        }
    }
}

pub fn try_generate_prime(range: &Range, rng: &mut ThreadRng) -> Option<BigUint> {
    let number = loop {
        let number = generate_initial(range, rng);

        if prime::simple_test(&number) {
            break number;
        }
    };

    if prime::miller_rabin_test(&number, rng) {
        Some(number)
    } else {
        None
    }
}

pub fn try_generate_safe_prime(range: &Range, rng: &mut ThreadRng) -> Option<BigUint> {
    let (number, low_number) = loop {
        let number = generate_initial(range, rng);
        let low_number = (&number - BigUint::one()) >> 1;

        if prime::simple_test(&number) && prime::simple_test(&low_number) {
            break (number, low_number);
        }
    };

    if prime::miller_rabin_test(&number, rng) && prime::miller_rabin_test(&low_number, rng) {
        Some(number)
    } else {
        None
    }
}

fn generate_initial(range: &Range, rng: &mut ThreadRng) -> BigUint {
    range.uniform.sample(rng) | &(BigUint::one() << (range.bit_count - 1)) | &BigUint::one()
}
