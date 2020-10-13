use std::io::Read;
use std::{fs::File, io::Error};

pub fn read_data(input_file: &str) -> Result<Vec<String>, Error> {
    let mut input = String::new();

    let mut file = File::open(input_file)?;
    file.read_to_string(&mut input)?;
    let lines = input.split('\n').map(|s: &str| s.to_string()).collect();
    Ok(lines)
}
