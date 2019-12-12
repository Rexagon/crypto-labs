use std::cmp::Ordering;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, Read};

use hashbrown::HashMap;

type Rates = Vec<(char, u64)>;
type Mapping = Vec<(char, char)>;

struct Table {
    characters: HashMap<char, u64>,
    count: u64,
}

impl Table {
    fn new() -> Self {
        Table {
            characters: HashMap::new(),
            count: 0,
        }
    }

    fn process_line(&mut self, line: &str) {
        for c in line.chars() {
            match c {
                'а'..='я' | 'ё' => {
                    self.record_character(c);
                }
                _ => (),
            }
        }
    }

    fn finalize(&mut self) -> Rates {
        let mut rates = self
            .characters
            .iter()
            .map(|(c, r)| (*c, *r))
            .collect::<Vec<(char, u64)>>();

        rates.sort_by(|(_, l), (_, r)| l.cmp(&r).reverse());

        rates
    }

    fn record_character(&mut self, c: char) {
        match self.characters.get_mut(&c) {
            Some(count) => {
                *count += 1;
            }
            None => {
                self.characters.insert(c, 1);
            }
        };

        self.count += 1;
    }
}

fn create_mapping(source: &Rates, target: &Rates) -> Mapping {
    target
        .iter()
        .zip(target.iter())
        .map(|((s, _), (t, _))| (*s, *t))
        .collect()
}

fn decrypt(input: &str, mapping: &Mapping) -> String {
    input
        .chars()
        .map(|c| match c {
            'а'..='я' | 'ё' => mapping
                .iter()
                .find(|&(old, _)| *old == c)
                .map(|&(_, new)| new)
                .unwrap_or(c),
            _ => c,
        })
        .collect()
}

fn read_path() -> String {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        std::process::exit(1);
    }

    args[1].clone()
}

fn main() -> Result<(), Error> {
    // Create source rates
    let source_rates = {
        let mut table = Table::new();

        let file = File::open(read_path())?;
        let buffer = BufReader::new(file);
        for line in buffer.lines() {
            table.process_line(line?.to_lowercase().as_str());
        }

        table.finalize()
    };

    // Read input
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer)?;

    // Create target rates
    let target_rates = {
        let mut table = Table::new();

        for line in buffer.lines() {
            table.process_line(&line.to_lowercase());
        }

        table.finalize()
    };

    println!(
        "{}",
        decrypt(
            buffer.as_str(),
            &create_mapping(&source_rates, &target_rates)
        )
    );

    Ok(())
}
