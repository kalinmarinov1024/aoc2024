use std::fs;

fn main() {
    let path = "../input.txt";

    let result = read_file_and_get_columns(&path).unwrap();

    print!("{result}")
}

fn read_file_and_get_columns(file_path: &str) -> Option<u32> {
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
