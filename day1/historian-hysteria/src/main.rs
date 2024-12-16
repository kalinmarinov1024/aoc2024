use std::fs;
use std::collections::HashMap;

fn main() {
    let path = "../input.txt";

    let part1 = part1(&path).unwrap();
    let part2 = part2(&path).unwrap();

    print!("{part2}");
}

fn part1(file_path: &str) -> Option<u32> {
    let data = fs::read_to_string(file_path).expect("Error reading file");

    let (mut a, mut b) = (Vec::new(), Vec::new());

    for line in data.lines() {
        let mut parts = line.split("   ");
        a.push(parts.next().unwrap().parse::<u32>().unwrap());
        b.push(parts.next().unwrap().parse::<u32>().unwrap());
    }

    a.sort_unstable();
    b.sort_unstable();
    
    Some (
        a.iter()
        .zip(b.iter())
        .map(|(a, b)| a.abs_diff(*b))
        .sum(),
    )
}

fn part2(file_path: &str) -> Option<u32> {
    let data = fs::read_to_string(file_path).expect("Error reading file");

    let mut a= Vec::new();
    let mut b = HashMap::new();

    for line in data.lines() {
        let mut parts = line.split("   ");

        let left_column: u32 = parts.next().unwrap().parse::<u32>().unwrap();
        let right_column: u32 = parts.next().unwrap().parse::<u32>().unwrap();

        a.push(left_column);

        let count: &mut u32 = b.entry(right_column).or_insert(0);
        *count += 1;
    }

    Some(
        a.iter()
        .map(|a| a * b.get(a).unwrap_or(&0))
        .sum()
    )
}
