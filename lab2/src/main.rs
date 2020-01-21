use {num_bigint::BigUint, rand::Rng};

use primes::{math, PrimeGenerator, Range};

fn main() {
    let mut rng = rand::thread_rng();
    let range = Range::new(50);

    // Alice side
    println!("Alice calculates values:");

    let p = range.generate_prime(&mut rng);
    let g = math::primitive_root_modulo(&p);
    println!("p: {}", p);
    println!("g: {}", g);

    let alice = Side::new(&p, &g, &range, &mut rng);
    println!("A: {}", &alice.secret_number);
    println!("a: {}", &alice.intermediate);

    // Transmission
    println!("\n---Alice sending values (p, g, a) to Bob---\n");

    // Bob side
    println!("Bob calculates values:");

    let bob = Side::new(&p, &g, &range, &mut rng);
    println!("B: {}", &bob.secret_number);
    println!("b: {}", &bob.intermediate);

    // Transmission
    println!("\n---Bob sending values (b) to Alice---\n");

    // Alice side
    let alice_key = alice.complete(&bob.intermediate);
    println!("Alice calculates key: {}", &alice_key);

    // Bob side
    let bob_key = bob.complete(&alice.intermediate);
    println!("Bob calculates key: {}", &bob_key);
}

struct Side {
    p: BigUint,
    secret_number: BigUint,
    intermediate: BigUint,
}

impl Side {
    fn new<R: Rng + ?Sized>(p: &BigUint, g: &BigUint, range: &Range, rng: &mut R) -> Self {
        let secret_number = range.generate_prime(rng);
        let intermediate = g.modpow(&secret_number, &p);

        Side {
            p: p.clone(),
            secret_number,
            intermediate,
        }
    }

    fn complete(&self, intermediate: &BigUint) -> BigUint {
        intermediate.modpow(&self.secret_number, &self.p)
    }
}
