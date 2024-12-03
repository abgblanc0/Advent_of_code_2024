use std::fs;

pub fn solution() {
    let input: String = fs::read_to_string("src/days/day01/input.txt").expect("Error");

    let mut left_list: Vec<i32> = Vec::new();
    let mut right_list: Vec<i32> = Vec::new();
    let mut result = 0;
    for line in input.lines() {
        if let Some((first, second)) = line.split_once(' ') {
            left_list.push(first.trim().parse().unwrap());
            right_list.push(second.trim().parse().unwrap());
        }
    }
    left_list.sort();
    right_list.sort();

    for (index, val) in left_list.iter().enumerate() {
        result += (val - right_list[index]).abs();
    }
    println!("1.1: {result}");
}
