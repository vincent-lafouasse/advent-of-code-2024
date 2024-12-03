use regex::Regex;
use std::fs;

const INPUT_PATH: &str = "input/data.txt";
const __TEST_INPUT_PATH: &str = "input/test.txt";

fn main() {
    let res: u32 = solve_part1(INPUT_PATH);
    println!("{res}");
}

fn solve_part1(path: &str) -> u32 {
    let data: String = fs::read_to_string(path).expect("Unable to read file");
    let pattern: Regex = Regex::new(r"mul\(\d+,\d+\)").unwrap();
    let couples: Vec<(u32, u32)> = pattern
        .find_iter(&data)
        .map(|m| m.as_str())
        .map(|s| s.strip_prefix("mul(").unwrap().strip_suffix(")").unwrap())
        .map(|s| {
            let parts: Vec<&str> = s.split(",").collect();
            let a = parts[0].parse().unwrap();
            let b = parts[1].parse().unwrap();
            (a, b)
        })
        .collect();

    couples.iter().map(|(a, b)| a * b).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(161, solve_part1(__TEST_INPUT_PATH));
    }
}
