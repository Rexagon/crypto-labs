mod prime_numbers;

use rand::prelude::ThreadRng;
use rand::distributions::{Distribution, Uniform};
use num_bigint::{BigUint, ToBigUint};

use prime_numbers::is_prime;

const BIT_COUNT: u64 = 40;

fn main() {
    let range = Uniform::new(0, 0x1 << BIT_COUNT);
    let mut rng = rand::thread_rng();

    for _ in 0..10 {
        println!("{:?}", generate_prime(&range, &mut rng));
    }
}

fn generate_prime(range: &Uniform<u64>, rng: &mut ThreadRng) -> Option<u64>
{
    let mut number;
    loop {
        number = generate_initial_number(range, rng);
        if is_prime(&number) {
            break;
        }
    }

    if test_number(rng, number) {
        Some(number)
    } else {
        None
    }
}

#[inline]
fn generate_initial_number(range: &Uniform<u64>, rng: &mut ThreadRng) -> u64 {
    range.sample(rng) | (0x1u64 << (BIT_COUNT - 1)) | 0x1u64
}

fn test_number(rng: &mut ThreadRng, p: u64) -> bool {
    let one: BigUint = 1.to_biguint().unwrap();
    let two: BigUint = 2.to_biguint().unwrap();

    let mut b = 1u64;
    let mut m = p >> 1;
    while m & 0x1u64 == 0 {
        m >>= 1;
        b += 1;
    }

    let range = Uniform::new(2, p - 2);

    let m: BigUint = m.to_biguint().unwrap();
    let p: BigUint = p.to_biguint().unwrap();
    let p_minus_one = p.clone() - one.clone();

    'outer: for _ in 0..5 {
        let a: BigUint = range.sample(rng).to_biguint().unwrap();
        let mut z = a.modpow(&m, &p);

        if z.eq(&one) || z.eq(&p_minus_one) {
            continue 'outer;
        }

        for _ in 0..b {
            z = z.modpow(&two, &p);

            if z.eq(&one) {
                return false;
            }

            if z.eq(&p_minus_one) {
                continue 'outer;
            }
        }

        return false;
    }

    true
}
