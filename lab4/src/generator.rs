use primes::Range;

use lab4::srp6::security_base::SecurityBase;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let bit_count = match args.len() {
        l if l > 1 => match args[1].parse::<usize>() {
            Ok(bitness) => bitness,
            Err(_) => {
                eprintln!("Bad argument!");
                std::process::exit(-1);
            }
        },
        _ => DEFAULT_BITNESS,
    };

    let mut rng = rand::thread_rng();
    let range = Range::new(bit_count);

    let security_base = SecurityBase::new(&range, &mut rng);

    println!("{}", security_base.to_string());
}

const DEFAULT_BITNESS: usize = 512;
