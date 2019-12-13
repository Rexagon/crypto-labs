use std::io::{Error, ErrorKind};

pub fn read_path() -> Result<String, Error> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        Err(Error::from(ErrorKind::InvalidInput))
    } else {
        Ok(args[1].clone())
    }
}
