use std::fs;

#[derive(Debug)]
struct Test {
  result: u64,
  nums: Vec<u64>
}

fn calibrate(result: u64, nums: &[u64]) -> bool {
  let Some(&num) = nums.first() else {
    return false;
  };
  (nums.len() == 1 && num == result) || 
  (result % num == 0 && calibrate(result / num, &nums[1..])) ||
  (result > num && calibrate(result - num, &nums[1..]))
}

pub fn solution() {
  let input = fs::read_to_string("src/days/day07/input.txt").unwrap();
  let tests:Vec<Test> = input.lines().map(|line| 
    {
    let (result, ops) = line.split_once(':').unwrap();
    let nums: Vec<u64> = ops.split(' ').filter(|n| !n.is_empty()).map(|n| n.parse().unwrap()).rev().collect();
    Test { result: result.parse::<u64>().unwrap(), nums}
    }
  ).collect();
  let mut result = 0;
  for test in tests {
    result += if calibrate(test.result, &test.nums) { test.result } else { 0 }
  }
  println!("7.1: {result}");
}