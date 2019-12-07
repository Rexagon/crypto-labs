use {
    num_bigint::{BigUint, ToBigUint},
    rand::Rng,
};

use primes::{math, PrimeGenerator, Range};

fn main() {
    let mut rng = rand::thread_rng();
    let range = Range::new(40);

    // Secret numbers
    let alice_number = range.generate_prime(&mut rng).to_biguint().unwrap();
    println!("Alice number... {}", alice_number);

    let bob_number = range.generate_prime(&mut rng).to_biguint().unwrap();
    println!("Bob number..... {}", bob_number);

    //
    println!("\nAlice sending values:");

    let (p, g) = generate_initial_values(&range, &mut rng);
    println!("p: {}", p);
    println!("g: {}", g);

    let alice_intermediate = g.modpow(&alice_number, &p);
    println!("A: {}", alice_intermediate);

    //
    println!("\nBob sending values:");

    let bob_intermediate = g.modpow(&bob_number, &p);
    println!("B: {}", bob_intermediate);

    //
    println!("\nAlice calculates key:");

    let alice_key = bob_intermediate.modpow(&alice_number, &p);
    println!("Ka: {}", alice_key);

    //
    println!("\nBob calculates key:");

    let bob_key = alice_intermediate.modpow(&bob_number, &p);
    println!("Kb: {}", bob_key);

    //
    println!("\nKa equals Kb is {:?}", alice_key.eq(&bob_key));
}

fn generate_initial_values<R: Rng + ?Sized>(range: &Range, rng: &mut R) -> (BigUint, BigUint) {
    let p = range.generate_prime(rng);

    (
        p.to_biguint().unwrap(),
        math::primitive_root_modulo(&p.to_biguint().unwrap()),
    )
}
