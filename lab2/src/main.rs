use num_bigint::{BigUint, ToBigUint};

use primes::generation::{Range, generate_prime};
use primes::math::primitive_root_modulo;

fn main() {
    let range = Range::new(40);

    // Secret numbers
    let alice_number = generate_prime(&range).to_biguint().unwrap();
    println!("Alice number... {}", alice_number);

    let bob_number = generate_prime(&range).to_biguint().unwrap();
    println!("Bob number..... {}", bob_number);

    //
    println!("\nAlice sending values:");

    let (p, g) = generate_initial_values(&range);
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


fn generate_initial_values(range: &Range) -> (BigUint, BigUint) {
    let p = generate_prime(&range);

    (p.to_biguint().unwrap(), primitive_root_modulo(p).to_biguint().unwrap())
}
