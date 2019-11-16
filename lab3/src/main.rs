use num_bigint::{BigUint, ToBigUint};

use primes::generation::{generate_prime, Range};
use primes::math::{least_common_multiple, greatest_common_divisor};

fn main() {
    let p = generate_prime(&Range::new(40)).to_biguint();
    let q = generate_prime(&Range::new(45)).to_biguint();

    let lambda = least_common_multiple(p - 1, q - 1);

    let e = 65537.to_biguint().unwrap();

    let p = p.to_biguint().unwrap();
    let q = q.to_biguint().unwrap();

    let n = p * q;

    println!("{}", greatest_common_divisor(e, lambda));

    //let d = mod_inverse(e, lambda);
}

/*
fn mod_inverse(a: BigUint, m: BigUint) -> Option<BigUint> {
    let g = greatest_common_divisor(a, m);

    if g != 1 {
        None
    } else {
        // If a and m are relatively prime, then modulo inverse
        // is a^(m-2) mode m
        cout << "Modular multiplicative inverse is "
            << power(a, m - 2, m);
    }
}
*/
