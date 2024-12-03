#![allow(unused)]

use regex::Regex;
use std::fs;

const INPUT_PATH: &str = "input/data.txt";
const __TEST_INPUT_PATH: &str = "input/test.txt";

fn main() {
    let res: u32 = solve_part1(__TEST_INPUT_PATH);
    println!("{res}");
}

fn solve_part1(path: &str) -> u32 {
    let data: String = fs::read_to_string(path).expect("Unable to read file");
    let pattern: Regex = Regex::new(r"mul(\d+,\d+)").unwrap();
    let matches: Vec<&str> = pattern.find_iter(&data).map(|m| m.as_str()).collect();
    dbg!(matches);

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(161, solve_part1(__TEST_INPUT_PATH));
    }
}
