use std::{collections::HashMap, fs, usize};

const INPUT_PATH: &str = "input.txt";

fn main() {
    println!("Hello, world!");
    let input = fs::read_to_string(INPUT_PATH).unwrap();

    println!("result: {}", print_queue_2(input));
}

fn print_queue_1(input: String) -> usize {
    let mut mid_elements = 0;
    let map = parse_upgrade_order(&input);

    for line in input.lines() {
        if line.contains('|') || line.is_empty() {
            continue;
        }

        let correct = is_line_corrent(&map, line.to_string());

        if correct {
            let numbers: Vec<usize> = line.split(',').map(|x| x.parse().unwrap()).collect();
            let mid_elem: usize = numbers.get(numbers.len() / 2).unwrap().to_owned();
            mid_elements += mid_elem;
        }
    }

    mid_elements
}

fn print_queue_2(input: String) -> usize {
    let mut mid_elements = 0;
    let map = parse_upgrade_order(&input);

    for line in input.lines() {
        if line.contains('|') || line.is_empty() {
            continue;
        }

        let correct = is_line_corrent(&map, line.to_string());

        if !correct {
            let numbers: Vec<usize> = line.split(',').map(|x| x.parse().unwrap()).collect();
            let mid_elem: usize = reorder_and_get_mid(&numbers, &map);
            mid_elements += mid_elem;
        }
    }

    mid_elements
}

// we know that by the upgrade rules, going to the right of the line, the number will have
// fewer matching next numbers.
fn reorder_and_get_mid(upgrade_order: &Vec<usize>, map: &HashMap<usize, Vec<usize>>) -> usize {
    let mut order: Vec<usize> = Vec::new();
    let order_size = upgrade_order.len();
    order.resize(order_size, 0);

    for current in upgrade_order.iter() {
        let mut idx_in_order = order_size;

        for next in upgrade_order.iter() {
            if current == next {
                continue;
            }

            //items who has more rules are closer to the start idx
            if map.contains_key(&current) && map.get(&current).unwrap().contains(&next) {
                idx_in_order -= 1;
            }
        }

        order[idx_in_order - 1] = current.clone();
    }

    order.get(order.len() / 2).unwrap().clone()
}

fn is_line_corrent(map: &HashMap<usize, Vec<usize>>, line: String) -> bool {
    let numbers: Vec<usize> = line.split(',').map(|x| x.parse().unwrap()).collect();

    let mut correct = true;
    for (x_pos, current) in numbers.iter().enumerate() {
        for y in numbers.iter().skip(x_pos + 1) {
            if map.contains_key(&current) && map.get(&current).unwrap().contains(&y) {
                continue;
            }

            correct = false;
            break;
        }
    }
    correct
}

fn parse_upgrade_order(input: &String) -> HashMap<usize, Vec<usize>> {
    let mut map: HashMap<usize, Vec<usize>> = HashMap::new();

    for line in input.lines() {
        if line.is_empty() {
            break;
        }

        let upgrade_order: Vec<&str> = line.split('|').collect();

        let number_before: usize = upgrade_order.get(0).unwrap().parse().unwrap();
        let number_after: usize = upgrade_order.get(1).unwrap().parse().unwrap();

        if map.contains_key(&number_before) {
            let vec = map.get_mut(&number_before).unwrap();
            vec.push(number_after);
        } else {
            let mut vec: Vec<usize> = Vec::new();
            vec.push(number_after);
            map.insert(number_before, vec);
        }
    }
    map
}
