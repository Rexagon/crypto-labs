use {
    num_bigint::{BigUint, ToBigUint},
    num_traits::cast::ToPrimitive
};


pub fn primitive_root_modulo(number: u64) -> u64 {

    let fact: Vec<BigUint> = factorize(number)
        .iter()
        .map(|v| v.to_biguint().unwrap())
        .collect();

    let fact_count = fact.len();

    let one = 1.to_biguint().unwrap();

    let phi: BigUint = (number - 1).to_biguint().unwrap();
    let number: BigUint = number.to_biguint().unwrap();

    let mut result: BigUint = 2.to_biguint().unwrap();
    while result < number {
        let mut valid = true;

        let mut i = 0;
        while i < fact_count && valid {
            let exponent = &phi / &fact[i];
            valid &= result.modpow(&exponent, &number) != one;

            i += 1;
        }

        if valid {
            return result.to_u64().unwrap();
        }

        result += &one;
    }

    unreachable!();
}


pub fn least_common_multiple(a: u64, b: u64) -> BigUint {
    if a == 0 || b == 0 {
        return 0.to_biguint().unwrap();
    }

    let g = greatest_common_divisor(a, b).to_biguint().unwrap();

    let a = a.to_biguint().unwrap();
    let b = b.to_biguint().unwrap();

    return a * b / g;
}


pub fn greatest_common_divisor(mut a: u64, mut b: u64) -> u64 {
    assert!(a != 0 && b != 0);

    let mut remainder;
    loop {
        remainder = a % b;
        a = b;
        b = remainder;

        if b == 0 {
            break a;
        }
    }
}


fn factorize(number: u64) -> Vec<u64> {
    let mut result= Vec::new();

    let mut n = number - 1;

    let mut i = 2;
    while i * i <= n {
        if n % i == 0 {
            result.push(i);

            while n % i == 0 {
                n /= i;
            }
        }

        i += 1;
    }

    if n > 1 {
        result.push(n);
    }

    result
}


