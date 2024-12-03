use regex::Regex;
use std::fs;

pub fn solution() {
  let input = fs::read_to_string("src/days/day03/input.txt").unwrap();
  let reops = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
  let mut result = 0;
  let operations: Vec<(i32, i32)> = reops.captures_iter(&input)
    .map(|cap| (cap[1].parse().unwrap(), cap[2].parse().unwrap()))
    .collect();
  for op in operations {
    result += op.0 * op.1;
  }
  println!("3.1: {result}");
}
