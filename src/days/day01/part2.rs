use std::{collections::HashMap, fs};

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
    right_list.sort();
    let mut list:HashMap<i32, i32> = HashMap::new();
    for value in right_list {
        *list.entry(value).or_insert(0) += 1;
    }
    for value in left_list {
        result += match list.get(&value) {
            Some(a) => value * a,
            None => 0
        }
    }
    println!("1.2: {result}")
}