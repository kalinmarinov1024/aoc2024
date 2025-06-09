use std::{fs, usize};

const INPUT_PATH: &str = "input.txt";

fn main() {
    println!("Hello, world!");

    let input: String = fs::read_to_string(INPUT_PATH).unwrap();
    println!("res: {}", part_two(input));
}

fn part_one(input: String) -> usize {
    let mut occurs = 0;

    let is_xmas = |s| s == "XMAS" || s == "SAMX";

    let input = parse_input(input);
    let xlen = input[0].len();
    let ylen = input.len();

    for x in 0..xlen - 3 {
        for y in 0..ylen {
            if is_xmas(format!(
                "{}{}{}{}",
                input[y][x],
                input[y][x + 1],
                input[y][x + 2],
                input[y][x + 3]
            )) {
                occurs += 1;
            }
        }
    }

    // Vertical
    for x in 0..xlen {
        for y in 0..ylen - 3 {
            if is_xmas(format!(
                "{}{}{}{}",
                input[y][x],
                input[y + 1][x],
                input[y + 2][x],
                input[y + 3][x]
            )) {
                occurs += 1;
            }
        }
    }

    // First diagonal
    for x in 0..xlen - 3 {
        for y in 0..ylen - 3 {
            if is_xmas(format!(
                "{}{}{}{}",
                input[y][x],
                input[y + 1][x + 1],
                input[y + 2][x + 2],
                input[y + 3][x + 3]
            )) {
                occurs += 1;
            }
        }
    }

    // Second diagonal
    for x in 3..xlen {
        for y in 0..ylen - 3 {
            if is_xmas(format!(
                "{}{}{}{}",
                input[y][x],
                input[y + 1][x - 1],
                input[y + 2][x - 2],
                input[y + 3][x - 3]
            )) {
                occurs += 1;
            }
        }
    }

    occurs
}

fn part_two(input: String) -> usize {
    let mut occurs = 0;

    let is_x_mas = |s1, s2| (s1 == "MAS" || s1 == "SAM") && (s2 == "SAM" || s2 == "MAS");

    let input = parse_input(input);
    let xlen = input[0].len();
    let ylen = input.len();

    for x in 0..xlen - 2 {
        for y in 0..ylen - 2 {
            if is_x_mas(
                format!(
                    "{}{}{}",
                    input[y][x],
                    input[y + 1][x + 1],
                    input[y + 2][x + 2]
                ),
                format!(
                    "{}{}{}",
                    input[y][x + 2],
                    input[y + 1][x + 1],
                    input[y + 2][x]
                ),
            ) {
                occurs += 1;
            }
        }
    }

    occurs
}

fn parse_input(input: String) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}
