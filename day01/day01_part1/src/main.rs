#![allow(dead_code)]

use std::fs::File;
use std::io::{self, BufRead, BufReader};

const INPUT_PATH: &'static str = "src/input1.txt";

fn main() -> io::Result<()> {
    let file = File::open(INPUT_PATH)?;
    let reader = BufReader::new(file);

    let mut list1: Vec<i32> = Vec::new();
    let mut list2: Vec<i32> = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Failed to read line from file");
        let (n1, n2): (i32, i32) = parse_line(&line).unwrap_or_else(|err| {
            panic!(
                "Failed to read line \"{}\"\nEncoutered error {}",
                line,
                err.repr()
            )
        });
        list1.push(n1);
        list2.push(n2);
    }

    list1.sort();
    list2.sort();

    // let result: i32 = std::iter::zip(list1.iter(), list2.iter()).map(|(n1, n2)|)

    Ok(())
}

#[derive(Debug)]
enum ParseError {
    NotANumber,
    InvalidLen,
}

impl ParseError {
    fn repr(&self) -> &'static str {
        match *self {
            ParseError::NotANumber => "Not a number",
            ParseError::InvalidLen => "Invalid len",
        }
    }
}

fn parse_line(line: &str) -> Result<(i32, i32), ParseError> {
    let parts: Vec<&str> = line.split_whitespace().collect();

    if parts.len() != 2 {
        return Err(ParseError::InvalidLen);
    }

    let n1 = parts[0].parse::<i32>();
    let n2 = parts[1].parse::<i32>();

    if n1.is_err() || n2.is_err() {
        Err(ParseError::NotANumber)
    } else {
        Ok((n1.unwrap(), n2.unwrap()))
    }
}
