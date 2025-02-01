use std::fs;

fn main() {
    let input = "../input.txt";
    
    let result_part_one = part1(input);
    println!("result: {}", result_part_one);
    
    let result_part_two = part2(input);
    println!("result: {}", result_part_two);
}

fn part1(file_path: &str) -> u32 {
    let mut safe_reports: u32 = 0;
    let data = fs::read_to_string(file_path).expect("err reading file");

    for line in data.lines() {
        let result: Vec<u32> = line.split_whitespace()
        .map(|s| s.parse::<u32>().unwrap())
        .collect();

        if is_monotonic(&result) {
            safe_reports += 1;
        }
    }

    return safe_reports;
}


fn part2(file_path: &str) -> u32 {
    let mut safe_reports: u32 = 0;
    let data = fs::read_to_string(file_path).expect("Err reading file.");

    for line in data.lines() {
        let result: Vec<u32> = line.split_whitespace()
        .map(|s| s.parse::<u32>().unwrap()).collect();

        if is_monotonic_with_skip(&result) {
            safe_reports += 1;
        }
    }

    return safe_reports;
}

fn is_monotonic(numbers: &Vec<u32>) -> bool {
    let mut prev: u32 = numbers[0];
    let starting_direction: bool = numbers[0] > numbers[1];

    for x in &numbers[1..] {
        let diff = prev.abs_diff(*x);

        if starting_direction == (prev > *x) && diff >= 1 && diff <=3 {
            prev = *x;
        } else {
            return false;
        }
    }

    return true;
}

fn is_monotonic_with_skip(numbers: &Vec<u32>) -> bool {
    let count = numbers.iter()
    .filter(|_| {
        is_monotonic(numbers) ||
        (0..numbers.len()).any(|i| {
            let mut numbers = numbers.clone();
            numbers.remove(i);
            is_monotonic(&numbers)
            })
    }).count();

    return count != 0;
}