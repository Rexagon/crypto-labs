use {
    hashbrown::HashMap,
    std::io::{Error, Read},
};

fn main() -> Result<(), Error> {
    // Create reference rates
    let reference_alphabet = lab1::create_alphabet_from_file::<char>(&lab1::read_path()?)?;

    // Read input
    let mut buffer = String::new();
    std::io::stdin().lock().read_to_string(&mut buffer)?;

    // Create target rates
    let target_alphabet = lab1::create_alphabet_from_string::<char>(&buffer);

    //
    let mapping = target_alphabet
        .iter()
        .zip(reference_alphabet.iter())
        .map(|(c, r)| (*c, *r))
        .collect::<HashMap<char, char>>();

    let decrypted = buffer
        .chars()
        .map(|c| match c {
            'а'..='я' | 'ё' => *mapping.get(&c).unwrap_or(&c),
            _ => c,
        })
        .collect::<String>();

    println!("{}", decrypted);

    Ok(())
}
