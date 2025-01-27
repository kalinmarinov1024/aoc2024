use std::fs;

fn main() {
    let input = "../input.txt";
    
    let result_part_one = part1(input);
    println!("result: {}", result_part_one);

}


fn part1(file_path: &str) -> u32 {
    let mut safe_reports: u32 = 0;
    let data = fs::read_to_string(file_path).expect("err reading file");

    for line in data.lines() {
        let result: Vec<u32> = line.split_whitespace()
        .map(|s| s.parse::<u32>().unwrap())
        .collect();

        let mut prev: u32 = result[0];
        let starting_direction: bool = result[0] > result[1];
        let mut success: bool = false;

        for x in &result[1..] {
            let diff = prev.abs_diff(*x); 

            if starting_direction == (prev > *x) && diff >= 1 && diff <=3 {
                prev = *x;
                success = true;
            } else {
                success = false;
                break;
            }    
        }

        if success {safe_reports += 1}
    }

    return safe_reports;
}