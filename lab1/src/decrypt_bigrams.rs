use {
    hashbrown::HashMap,
    std::io::{Error, Read, Write},
};

use lab1::{Bigram, ParsingItem};

fn main() -> Result<(), Error> {
    // Create reference rates
    let reference_alphabet = lab1::create_alphabet_from_file::<Bigram>(&lab1::read_path()?)?;

    // Read input
    let mut buffer = String::new();
    std::io::stdin().lock().read_to_string(&mut buffer)?;

    // Create target rates
    let target_alphabet = lab1::create_alphabet_from_string::<Bigram>(&buffer);

    // Create mappings
    let half_bigrams_mapping = create_mapping(&reference_alphabet, &target_alphabet, false);
    let full_bigrams_mapping = create_mapping(&reference_alphabet, &target_alphabet, true);

    // Decrypt text
    for line in buffer.lines() {
        for item in lab1::Parser::new(&line.to_lowercase()) {
            let bigram = match item {
                ParsingItem::Parsed(bigram) => bigram,
                ParsingItem::Unparsed(chars) => {
                    std::io::stdout().write(chars.iter().collect::<String>().as_bytes())?;
                    continue;
                }
            };

            let decrypted = if bigram.is_half() {
                half_bigrams_mapping.get(&bigram).unwrap_or(&bigram)
            } else {
                full_bigrams_mapping.get(&bigram).unwrap_or(&bigram)
            };

            match decrypted {
                Bigram::Full(l, r) => std::io::stdout().write(format!("{}{}", l, r).as_bytes())?,
                Bigram::Half(c) => std::io::stdout().write(format!("{}", c).as_bytes())?,
            };
        }

        std::io::stdout().write("\n".as_bytes())?;
        std::io::stdout().flush()?;
    }

    std::io::stdout().flush()?;
    Ok(())
}

fn create_mapping(
    reference_alphabet: &Vec<Bigram>,
    target_alphabet: &Vec<Bigram>,
    full: bool,
) -> HashMap<Bigram, Bigram> {
    let predicate = move |b: &&Bigram| if full { b.is_full() } else { b.is_half() };

    target_alphabet
        .iter()
        .filter(predicate)
        .zip(reference_alphabet.iter().filter(predicate))
        .map(|(c, r)| (*c, *r))
        .collect()
}
