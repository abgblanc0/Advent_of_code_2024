use std::fs;

fn split(num: u128) -> (u128, u128) {
  let s = num.to_string();
  let midpoint = s.len() / 2;
  let (left, right) = s.split_at(midpoint);
  let left: u128 = left.parse().unwrap();
  let right: u128 = right.parse().unwrap();
  (left, right)
}

fn blink(stones: &mut Vec<u128>) {
  let mut index = 0;
  while index < stones.len() {
    if stones[index] == 0 { stones[index] = 1; }
    else if (stones[index].ilog10() + 1)%2 == 0 { 
      let (left, right) = split(stones[index]);
      stones[index] = right;
      stones.insert(index, left);
      index += 1;
    }
    else { stones[index] *= 2024 };
    index += 1;
  }
}

pub fn solution() {
  let mut stones: Vec<u128> = fs::read_to_string("src/days/day11/input.txt").unwrap().split(' ').map(|n| n.parse().unwrap()).collect();
  
  for _ in 0..25 {
    blink(&mut stones);
  } 
  println!("11.1: {}", stones.iter().count());
}