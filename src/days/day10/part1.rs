use std::{collections::HashSet, fs};

const DIRECTIONS: [(isize, isize); 4] = [
  (-1,  0),
  (1 ,  0),
  (0 , -1),
  (0 ,  1),
];

fn get_num(map: &Vec<Vec<u32>>, pos: (isize, isize)) -> Option<u32> {

  if let Some(v) = map.get(pos.0 as usize) {
    match v.get(pos.1 as usize) {
      Some(v) => return Some(*v),
      None => return None
    }
  }
  None
}

fn trail(map: &Vec<Vec<u32>>, pos: (usize, usize), num: u32) -> HashSet<(usize, usize)> {

  let mut result = HashSet::new();

  for dir in DIRECTIONS {
    let pos = (pos.0 as isize + dir.0, pos.1 as isize + dir.1);
    if let Some(v) = get_num(map, pos) {
      if v == num + 1 {
        result.extend(trail(map, (pos.0 as usize, pos.1 as usize), v));
      }
    }
  }
  if num == 9 { 
    result.insert(pos);  
  } 
  result
}

pub fn solution() {
  let map: Vec<Vec<u32>> = fs::read_to_string("src/days/day10/input.txt")
    .unwrap()
    .lines()
    .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
    .collect();

  let mut result = 0;
  for x in 0..map.len() {
    for y in 0..map[x].len() {
      if map[x][y] == 0 {
        result += trail(&map,(x,y), 0).iter().count();
      }
    }
  }
  println!("10.1: {result}")
}