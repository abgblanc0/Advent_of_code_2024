use regex::Regex;
use std::fs;

pub fn solution() {
  let input = fs::read_to_string("src/days/day03/input.txt").unwrap();
  let re = Regex::new(r"(do\(\)|don't\(\))[^d]+").unwrap();
  let reops = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
  let asd: Vec<&str> = re.find_iter(&input)
    .map(|mat| mat.as_str())
    .collect();
  let ax: Vec<&str> = asd.iter().filter(|s| s.contains("do()")).map(|&s| s).collect();
  let mut result = 0;
  let operations: Vec<(i32, i32)> = reops.captures_iter(&ax.join(""))
    .map(|cap| (cap[1].parse().unwrap(), cap[2].parse().unwrap()))
    .collect();

  for op in operations {
    result += op.0 * op.1;
  }

  println!("3.2: {result}");
}