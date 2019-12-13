use std::io::{Error, ErrorKind, Read};

fn main() -> Result<(), Error> {
    // Create reference rates
    let reference_alphabet = lab1::create_alphabet_from_file(&read_path()?)?;

    // Read input
    let mut buffer = String::new();
    std::io::stdin().lock().read_to_string(&mut buffer)?;

    // Create target rates
    let target_alphabet = lab1::create_alphabet_from_string(&buffer);

    //
    let mapping = lab1::create_mapping(&reference_alphabet, &target_alphabet);

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
