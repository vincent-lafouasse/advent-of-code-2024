use std::fs::File;
use std::io::{self, BufReader, BufRead};

const INPUT_PATH: &'static str = "input1.txt";

fn main() -> io::Result<()> {
    let file = File::open(INPUT_PATH)?;
    let reader = BufReader::new(file);

    let mut list1: Vec<u32> = Vec::new();
    let mut list2: Vec<u32> = Vec::new();

    for line in reader.lines() {
    }

    Ok(())
}

enum ParseError {
    NotANumber,
    InvalidLen,
}

fn parse_line(line: &str) -> Result<(u32, u32), ParseError> {
    Ok((0, 0))
}
