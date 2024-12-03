#![allow(dead_code)]
#![allow(unused)]

use std::fs::File;
use std::io::{BufRead, BufReader};

const INPUT_PATH: &str = "input/data.txt";
const __TEST_INPUT_PATH: &str = "input/test.txt";

fn main() {
    let file = File::open(__TEST_INPUT_PATH).expect("Failed to load input file");
    let reader = BufReader::new(file);

    let mut n_safe = 0;

    for line in reader.lines() {
        let line: String = line.expect("Failed to read line from file");
        let level: Vec<i32> = parse_line(&line);
        let differences: Vec<i32> = level
            .windows(2)
            .map(|window| window[1] - window[0])
            .collect();
        if differences.iter().all(|x| *x > 0 && *x < 4)
            || differences.iter().all(|x| *x > -4 && *x < 0)
        {
            n_safe += 1;
        }
    }

    println!("{n_safe}");
}

fn parse_line(line: &str) -> Vec<i32> {
    line.split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect()
}
