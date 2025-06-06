use regex::Regex;
use std::fs;

fn main() {
    println!("Hello, world!");

    let input = "input.txt";
    println!("answer: {}", part_two(input))
}

pub fn part_one(input: &str) -> u32 {
    let data = fs::read_to_string(input).unwrap();

    // let pattern = r"mul\(\d\(+\),\s*\d\(+\)\)"; // Corrected pattern
    let pattern = r"mul\((\d+),\s*(\d+)\)"; // Matches mul(...) with one or more digits
    let regex = Regex::new(pattern).unwrap(); // Create the regex object

    let mut total = 0;
    for mat in regex.captures_iter(&data) {
        let first = mat[1].parse::<u32>().unwrap();
        let second = mat[2].parse::<u32>().unwrap();
        let mul_result = first * second;

        total += mul_result;
    }

    return total;
}

//regex to turn on/off  based on do/don't
pub fn part_two(input: &str) -> usize {
    const RE_ANCHORED: &str = r"^mul\((\d+),(\d+)\)";
    let data = fs::read_to_string(input).unwrap();

    let re = Regex::new(RE_ANCHORED).unwrap();
    let mut sum: usize = 0;
    let mut enabled = true;

    for line in data.lines() {
        for i in 0..line.len() {
            if line[i..].starts_with(r"don't()") {
                enabled = false;
            } else if line[i..].starts_with(r"do()") {
                enabled = true;
            } else if line[i..].starts_with(r"mul(") && enabled {
                if let Some(cap) = re.captures(&line[i..]) {
                    let x: usize = cap.get(1).unwrap().as_str().parse().unwrap();
                    let y: usize = cap.get(2).unwrap().as_str().parse().unwrap();
                    sum = sum + x * y;
                }
            }
        }
    }
    sum
}
