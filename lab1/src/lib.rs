pub mod parser;
pub mod stuff;

use {
    hashbrown::HashMap,
    std::{
        fs::File,
        io::{BufRead, BufReader, Error},
    },
};

pub use crate::{
    parser::{Bigram, Parser, ParsingItem},
    stuff::*,
};

pub fn encrypt(message: String, shift: usize) -> String {
    let alphabet_upper: &str = "АБВГДЕЁЖЗИЙКЛМНОПРСТУФХЦЧШЩЪЫЬЭЮЯ";
    let alphabet_lower: &str = "абвгдеёжзийклмнопрстуфхцчшщъыьэюя";
    let alphabet_size = alphabet_upper.chars().count();

    let shift = shift % alphabet_size;
    let transform = |c: &char, alphabet: &str| {
        let position = alphabet.chars().position(|v| v == *c).unwrap();
        alphabet
            .chars()
            .nth((position + shift) % alphabet_size)
            .unwrap()
    };

    message
        .chars()
        .map(|c| match c {
            'А'..='Я' | 'Ё' => transform(&c, alphabet_upper),
            'а'..='я' | 'ё' => transform(&c, alphabet_lower),
            _ => c,
        })
        .collect()
}

pub fn create_alphabet_from_file<T>(path: &str) -> Result<Vec<T>, Error>
where
    T: Copy,
    HashMap<T, u64>: SymbolTable,
{
    let mut table = HashMap::new();

    let file = File::open(path)?;
    let buffer = BufReader::new(file);

    for line in buffer.lines() {
        if let Ok(line) = line {
            table.process_line(&line.to_lowercase());
        }
    }

    Ok(convert_rates(&table))
}

pub fn create_alphabet_from_string<T>(string: &str) -> Vec<T>
where
    T: Copy,
    HashMap<T, u64>: SymbolTable,
{
    let mut table = HashMap::new();

    for line in string.lines() {
        table.process_line(&line.to_lowercase());
    }

    convert_rates(&table)
}

pub trait SymbolTable {
    fn process_line(&mut self, line: &str);
}

impl SymbolTable for HashMap<char, u64> {
    fn process_line(&mut self, line: &str) {
        for c in line.chars() {
            match c {
                'а'..='я' | 'ё' => match self.get_mut(&c) {
                    Some(count) => {
                        *count += 1;
                    }
                    None => {
                        self.insert(c, 1);
                    }
                },
                _ => (),
            }
        }
    }
}

impl SymbolTable for HashMap<Bigram, u64> {
    fn process_line(&mut self, line: &str) {
        for item in Parser::new(line) {
            let bigram = match item {
                ParsingItem::Parsed(bigram) => bigram,
                _ => continue,
            };

            match self.get_mut(&bigram) {
                Some(count) => {
                    *count += 1;
                }
                None => {
                    self.insert(bigram, 1);
                }
            }
        }
    }
}

fn convert_rates<T: Copy>(table: &HashMap<T, u64>) -> Vec<T> {
    let mut rates = table.iter().map(|(c, r)| (*c, *r)).collect::<Vec<_>>();
    rates.sort_by(|(_, l), (_, r)| l.cmp(r).reverse());
    rates.iter().map(|(c, _)| *c).collect()
}
