mod rsa;

use num_bigint::BigUint;

fn main() {
    let mut rng = rand::thread_rng();
    let e = BigUint::from(65537u64);

    let (public_key, private_key) = rsa::generate_keys(&e, &mut rng);

    let test_number = BigUint::from(123456u64);
    println!("Origin: {}", &test_number);

    let encrypted = rsa::encrypt(&test_number, &public_key);
    println!("Encrypted: {}", &encrypted);

    let decrypted = rsa::decrypt(&encrypted, &private_key);
    println!("Decrypted: {}", &decrypted);
}
