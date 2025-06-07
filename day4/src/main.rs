use std::fs;

const INPUT_PATH: &str = "input.txt";
const XMAS_LENGTH: usize = 4;

fn main() {
    println!("Hello, world!");

    let input: String = fs::read_to_string(INPUT_PATH).unwrap();
    println!("res: {}", part_one(input));
}

fn part_one(input: String) -> usize {
    let mut occurs = 0;
    let lines = input.lines().count();

    for (idx, line) in input.lines().enumerate() {
        for (c_idx, _) in line.chars().enumerate() {
            let mut vec: Vec<char> = Vec::new();
            occurs += dfs(
                &input,
                idx,
                c_idx as i32,
                &mut vec,
                lines,
                line.len() as i32,
            );
        }
    }

    occurs
}

fn dfs(
    input: &String,
    line_idx: usize,
    char_idx: i32,
    uniques: &mut Vec<char>,
    max_lines: usize,
    line_chars: i32,
) -> usize {
    if line_idx == max_lines || char_idx == -1 || char_idx == line_chars {
        return 0;
    }

    let curr_char = input
        .lines()
        .nth(line_idx)
        .unwrap()
        .chars()
        .nth(char_idx as usize)
        .unwrap();

    if uniques.contains(&curr_char) {
        return 0;
    }

    uniques.push(curr_char);

    if uniques.len() == XMAS_LENGTH {
        return 1;
    }

    return dfs(
        &input,
        line_idx,
        char_idx + 1,
        &mut uniques.clone(),
        max_lines,
        line_chars,
    ) + dfs(
        &input,
        line_idx + 1,
        char_idx - 1,
        &mut uniques.clone(), //copies, as otherwise it gets filled by other dfs branches
        max_lines,
        line_chars,
    ) + dfs(
        &input,
        line_idx + 1,
        char_idx,
        &mut uniques.clone(),
        max_lines,
        line_chars,
    ) + dfs(
        input,
        line_idx + 1,
        char_idx + 1,
        &mut uniques.clone(),
        max_lines,
        line_chars,
    );
}
