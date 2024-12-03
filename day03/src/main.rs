use std::fs::File;
use std::io::{BufRead, BufReader};

const INPUT_PATH: &str = "input/data.txt";
const __TEST_INPUT_PATH: &str = "input/test.txt";

fn main() {}

fn solve_part1(path: &str) {
    let file = File::open(path).expect("Failed to load input file");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line: String = line.expect("Failed to read line from file");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        //assert_eq!(2, solve_part1(__TEST_INPUT_PATH));
        solve_part1(__TEST_INPUT_PATH);
    }
}
