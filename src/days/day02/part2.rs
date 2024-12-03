use std::fs;

fn is_increasing(vec: &[i32]) -> bool {
  vec.windows(2).all(|w| check_inc(w))
}

fn is_decreasing(vec: &[i32]) -> bool {
  vec.windows(2).all(|w| check_dec(w))
}

fn check_inc(w: &[i32]) -> bool {
  w[0] < w[1] && (w[0] - w[1]).abs() < 4
}

fn check_dec(w: &[i32]) -> bool {
  w[0] > w[1] && (w[0] - w[1]).abs() < 4
}

fn check(nums: &mut Vec<i32>) {
  let aux = nums.clone();
  let mut wrong_inc: Vec<(usize, &[i32])> = Vec::new();
  let mut wrong_dec: Vec<(usize, &[i32])> = Vec::new();
  for (index, pair) in aux.windows(2).enumerate() {
    if !check_inc(pair) {
      wrong_inc.push((index, pair));
    }
    if !check_dec(pair) {
      wrong_dec.push((index, pair));
    }
  }
  
  let dec = wrong_dec.len();
  let asc = wrong_inc.len();
  if (dec < 3 || asc < 3) && dec != 0 && asc != 0 {
    if dec < asc {
      if dec == 1 {
        let (key, _) = wrong_dec[0];
        let mut x = nums.clone(); x.remove(key);
        if is_decreasing(&x){
          *nums = x;
        }
        else {
          nums.remove(key + 1);
        }
      }
      else {
        nums.remove(wrong_dec.last().unwrap().0);
      }
    }
    else {
      if asc == 1 {
        let (key, _) = wrong_inc[0];
        let mut x = nums.clone(); x.remove(key);
        if is_increasing(&x) {
          *nums = x;
        }
        else {
          nums.remove(key + 1);
        }
      }
      else {
        nums.remove(wrong_inc.last().unwrap().0);
      }
    }
  }
}

pub fn solution() {
  let input = fs::read_to_string("src/days/day02/input.txt").unwrap();
  let mut result = 0;
  for line in input.lines() {
    let mut nums: Vec<i32> = line.split_whitespace().filter_map(|s| s.parse::<i32>().ok()).collect();
    check(&mut nums);
    result += if is_decreasing(&nums) || is_increasing(&nums) {1} else {0};
  }

  println!("2.2: {result}");
}