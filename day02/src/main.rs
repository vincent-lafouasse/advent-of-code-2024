#![allow(dead_code)]
#![allow(unused)]

use std::fs::File;
use std::io::{BufRead, BufReader};

const INPUT_PATH: &str = "input/data.txt";
const __TEST_INPUT_PATH: &str = "input/test.txt";

fn main() {
    let file = File::open(__TEST_INPUT_PATH).expect("Failed to load input file");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.expect("Failed to read line from file");
        let level = parse_line(&line);
        dbg!(level);
    }
}

fn parse_line(line: &str) -> Vec<u32> {
    line.split_whitespace()
        .map(|s| s.parse::<u32>().unwrap())
        .collect()
}

enum Direction {
    Increasing,
    Decreasing,
}

fn level_is_safe(level: &[u32]) -> bool {
    let mut buffer = level[0];
    let mut it = level.iter().next();

    true
}
