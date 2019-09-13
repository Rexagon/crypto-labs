use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

use hashbrown::HashMap;

struct State {
    characters: HashMap<char, u64>,
    count: u64
}

impl State {
    fn new() -> Self {
        State {
            characters: HashMap::new(),
            count: 0
        }
    }

    fn process_line(&mut self, line: String) {
        for c in line.to_lowercase().chars() {
            match c {
                'а'..='я' | 'ё' => {
                    self.record_character(c);
                },
                _ => ()
            }
        }
    }

    fn record_character(&mut self, c: char) {
        match self.characters.get_mut(&c) {
            Some(count) => {
                *count += 1;
            },
            None => {
                self.characters.insert(c, 1);
            }
        };

        self.count += 1;
    }

    fn get_result(&self) -> Vec<(&char, &u64)> {
        let mut result: Vec<(&char, &u64)> = self.characters.iter().collect();
        result.sort_by_key(|v| v.1);

        return result;
    }
}

fn main() -> Result<(), Error> {
    let file = File::open(read_path())?;
    let buffer = BufReader::new(file);

    let mut state = State::new();
    for line in buffer.lines() {
        state.process_line(line?);
    }

    let pairs = state.get_result();

    println!("Found {} characters", state.count);
    println!("Dictionaty:");    
    for (character, character_count) in pairs {
        println!("{}: {}", character, (*character_count as f64) / (state.count as f64));
    }

    Ok(())
}

fn read_path() -> String {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        std::process::exit(1);
    }

    args[1].clone()
}