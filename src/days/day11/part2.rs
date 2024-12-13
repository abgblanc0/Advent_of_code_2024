use std::fs;
use memoize::memoize;

fn split(n: u128) -> (u128, u128) {
    let digits = n.to_string();
    let mid = digits.len() / 2;
    let left: u128 = digits[..mid].parse().unwrap_or(0);
    let right: u128 = digits[mid..].parse().unwrap_or(0);
    (left, right)
}

#[memoize]
fn count(stone: u128, steps: i32) -> u128 {
    if steps == 0 { return 1 }
    if stone == 0 { return count(1, steps - 1) }
    if (stone.ilog10() + 1)%2 == 0 {
        let (left, right) = split(stone);
        return count(left, steps - 1) + count(right, steps - 1)
    }
    count(stone * 2024, steps - 1)
}

pub fn solution() {
  let stones: Vec<u128> = fs::read_to_string("src/days/day11/input.txt").unwrap().split(' ').map(|n| n.parse().unwrap()).collect();
  let mut result = 0;
  for stone in stones {
    result += count(stone, 75);
  }
  println!("11.2: {}", result);
}
