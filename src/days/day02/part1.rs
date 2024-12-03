use std::fs;

fn is_increasing(vec: &[i32]) -> bool {
    vec.windows(2).all(|w| w[0] < w[1] && (w[0] - w[1]).abs() > 0 && (w[0] - w[1]).abs() < 4)
}

fn is_decreasing(vec: &[i32]) -> bool {
    vec.windows(2).all(|w| w[0] > w[1] && (w[0] - w[1]).abs() > 0 && (w[0] - w[1]).abs() < 4)
}

pub fn solution() {
    let input = fs::read_to_string("src/days/day02/input.txt").unwrap();
    let mut result: i32 = 0;
    for line in input.lines() {
        let nums: Vec<i32> = line.split_whitespace().filter_map(|s| s.parse::<i32>().ok()).collect();
        result += if is_decreasing(&nums) || is_increasing(&nums) {1} else {0}
    }
    println!("2.1: {result}");
}
