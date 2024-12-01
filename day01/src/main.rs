use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[allow(unused)]
const INPUT_PATH: &str = "input/data.txt";
#[allow(unused)]
const TEST_INPUT_PATH: &str = "input/test.txt";

fn main() -> io::Result<()> {
    println!("{}", solve_part2(INPUT_PATH));

    Ok(())
}

#[allow(unused)]
fn solve_part1(path: &str) -> u32 {
    let (mut list1, mut list2) = load_input(path);

    list1.sort();
    list2.sort();

    list1
        .iter()
        .zip(list2.iter())
        .map(|(n1, n2)| if n1 > n2 { n1 - n2 } else { n2 - n1 })
        .sum()
}

#[allow(unused)]
fn solve_part2(path: &str) -> u32 {
    let (list1, list2) = load_input(path);

    let mut occurences: HashMap<u32, u32> = HashMap::new();
    for e in list1.iter() {
        if !occurences.contains_key(&e) {
            occurences.insert(*e, count_occurences(*e, &list2));
        }
    }

    list1.iter().map(|e| e * occurences[e]).sum()
}

fn count_occurences(n: u32, data: &[u32]) -> u32 {
    data.iter().map(|e| if *e == n { 1 } else { 0 }).sum()
}

fn load_input(path: &str) -> (Vec<u32>, Vec<u32>) {
    let file = File::open(path).expect("Failed to load input file");
    let reader = BufReader::new(file);

    let mut list1: Vec<u32> = Vec::new();
    let mut list2: Vec<u32> = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Failed to read line from file");
        let (n1, n2): (u32, u32) = parse_line(&line).unwrap_or_else(|err| {
            panic!(
                "Failed to read line \"{}\"\nEncoutered error {}",
                line,
                err.repr()
            )
        });
        list1.push(n1);
        list2.push(n2);
    }

    (list1, list2)
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

fn parse_line(line: &str) -> Result<(u32, u32), ParseError> {
    let parts: Vec<&str> = line.split_whitespace().collect();

    if parts.len() != 2 {
        return Err(ParseError::InvalidLen);
    }

    let n1 = parts[0].parse::<u32>();
    let n2 = parts[1].parse::<u32>();

    if n1.is_err() || n2.is_err() {
        Err(ParseError::NotANumber)
    } else {
        Ok((n1.unwrap(), n2.unwrap()))
    }
}
