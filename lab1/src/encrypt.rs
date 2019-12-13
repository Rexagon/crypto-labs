use std::io::{BufRead, Error, Write};

fn main() -> Result<(), Error> {
    let shift = read_shift(10);

    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout();

    for line in stdin.lock().lines() {
        stdout.write_all(lab1::encrypt(line?, shift).as_bytes())?;
    }

    stdout.flush().unwrap();

    Ok(())
}

fn read_shift(default: usize) -> usize {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        return default; // default shift
    }

    args[1].parse().unwrap_or(default)
}
