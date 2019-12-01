use {
    num_bigint::BigUint,
    num_traits::{One, Zero},
};

pub fn primitive_root_modulo(number: &BigUint) -> BigUint {
    let fact: Vec<BigUint> = factorize(number);

    let phi: BigUint = number - BigUint::one();

    let mut result: BigUint = BigUint::from(2u64);
    while &result < number {
        let mut valid = true;

        let mut i = 0;
        while i < fact.len() && valid {
            let exponent = &phi / &fact[i];
            valid &= result.modpow(&exponent, &number) != BigUint::one();

            i += 1;
        }

        if valid {
            return result;
        }

        result += BigUint::one();
    }

    unreachable!();
}

fn factorize(number: &BigUint) -> Vec<BigUint> {
    let mut result = Vec::new();

    let mut n = number - BigUint::one();

    let mut i = BigUint::from(2u64);
    while &i * &i <= n {
        if (&n % &i).is_zero() {
            result.push(i.clone());

            loop {
                n /= &i;

                if (&n % &i).is_zero() {
                    break;
                }
            }
        }

        i += BigUint::one();
    }

    if n > BigUint::one() {
        result.push(n);
    }

    result
}
