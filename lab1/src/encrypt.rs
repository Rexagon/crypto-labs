use std::env;
use std::io::{Write, BufRead, Error};

fn encrypt(message: String, shift: usize) -> String {
    let alphabet_upper: &str = "АБВГДЕЁЖЗИЙКЛМНОПРСТУФХЦШЩЪЫЬЭЮЯ";
    let alphabet_lower: &str = "абвгдеёжзийклмнопрстуфхцшщъыьэюя";
    let alphabet_size = alphabet_upper.chars().count();

    let shift = shift % alphabet_size;
    let transform = |c: &char, alphabet: &str| {
        let position = alphabet.chars().position(|v| v == *c).unwrap();
        alphabet.chars().nth((position + shift) % alphabet_size).unwrap()
    };

    message.chars().map(|c| {
        match c {
            'А'..='Я' | 'Ё' => transform(&c, alphabet_upper),
            'а'..='я' | 'ё' => transform(&c, alphabet_lower),
            _ => c
        }
    }).collect()
}

fn read_shift(default: usize) -> usize {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return default; // default shift
    }

    args[1].parse().unwrap_or(default)
}

fn main() -> Result<(), Error> {
    let shift = read_shift(10);

    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout();

    for line in stdin.lock().lines() {
        stdout.write_all(encrypt(line?, shift).as_bytes()).unwrap()
    }

    stdout.flush().unwrap();

    Ok(())
}