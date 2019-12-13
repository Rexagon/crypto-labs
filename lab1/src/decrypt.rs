use {
    hashbrown::HashMap,
    std::{
        fs::File,
        io::{BufRead, BufReader, Error, ErrorKind, Read},
    },
};

fn main() -> Result<(), Error> {
    // Create reference rates
    let reference_alphabet = create_alphabet_from_file(&read_path()?)?;

    // Read input
    let mut buffer = String::new();
    std::io::stdin().lock().read_to_string(&mut buffer)?;

    // Create target rates
    let target_alphabet = create_alphabet_from_string(&buffer);

    //
    let mapping = create_mapping(&reference_alphabet, &target_alphabet);

    let decrypted = buffer
        .chars()
        .map(|c| match c {
            'а'..='я' | 'ё' => mapping
                .iter()
                .find(|&(old, _)| *old == c)
                .map(|&(_, new)| new)
                .unwrap_or(c),
            _ => c,
        })
        .collect::<String>();

    println!("{}", decrypted);

    Ok(())
}

fn read_path() -> Result<String, Error> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        Err(Error::from(ErrorKind::InvalidInput))
    } else {
        Ok(args[1].clone())
    }
}

fn create_alphabet_from_file(path: &str) -> Result<Vec<char>, Error> {
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

fn create_alphabet_from_string(string: &str) -> Vec<char> {
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

fn create_mapping<T: Copy>(reference_alphabet: &Vec<T>, target_alphabet: &Vec<T>) -> Vec<(T, T)> {
    target_alphabet
        .iter()
        .zip(reference_alphabet.iter())
        .map(|(c, r)| (*c, *r))
        .collect()
}
