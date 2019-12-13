mod bigram;

use {
    hashbrown::HashMap,
    std::{
        fs::File,
        io::{BufRead, BufReader, Error},
    },
};

use crate::bigram::{Bigram, Bigrams};

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

pub fn create_mapping<T: Copy>(
    reference_alphabet: &Vec<T>,
    target_alphabet: &Vec<T>,
) -> Vec<(T, T)> {
    target_alphabet
        .iter()
        .zip(reference_alphabet.iter())
        .map(|(c, r)| (*c, *r))
        .collect()
}

pub fn create_alphabet_from_file(path: &str) -> Result<Vec<char>, Error> {
    let mut table = HashMap::new();

    let file = File::open(path)?;
    let buffer = BufReader::new(file);

    for line in buffer.lines() {
        if let Ok(line) = line {
            table.process_line(&line.to_lowercase());
        }
    }

    Ok(table.generate_alphabet())
}

pub fn create_alphabet_from_string(string: &str) -> Vec<char> {
    let mut table = HashMap::new();

    for line in string.lines() {
        table.process_line(&line.to_lowercase());
    }

    table.generate_alphabet()
}

trait SymbolTable<T> {
    fn process_line(&mut self, line: &str);
    fn generate_alphabet(&self) -> Vec<T>;
}

impl SymbolTable<char> for HashMap<char, u64> {
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

    fn generate_alphabet(&self) -> Vec<char> {
        let mut rates = self.iter().map(|(c, r)| (*c, *r)).collect::<Vec<_>>();

        rates.sort_by(|(_, l), (_, r)| l.cmp(r).reverse());

        rates.iter().map(|(c, _)| *c).collect()
    }
}

impl SymbolTable<Bigram> for HashMap<Bigram, u64> {
    fn process_line(&mut self, line: &str) {
        for bigram in Bigrams::new(line) {
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

    fn generate_alphabet(&self) -> Vec<Bigram> {
        unimplemented!()
    }
}
