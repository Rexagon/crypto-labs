use primes;

fn main() {
    let range = primes::Range::new(40);

    for _ in 0..10 {
        println!("{:?}", primes::generate(&range));
    }
}
