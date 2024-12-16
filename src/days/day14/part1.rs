use std::{fs, ops::Range};

const BOUNDS: (i32, i32) = (101, 103);

#[derive(Debug)]
struct Robot {
  pos: (i32, i32),
  vel: (i32, i32)
}

impl Robot {
  fn step(&mut self) {
    self.pos.0 = (self.pos.0 + self.vel.0).rem_euclid(BOUNDS.0);
    self.pos.1 = (self.pos.1 + self.vel.1).rem_euclid(BOUNDS.1);
  }
}

fn in_bounds(mut range: (Range<i32>, Range<i32>), pos: (i32, i32)) -> bool {
  range.0.any(|i| i == pos.0) && range.1.any(|i| i == pos.1)
}

pub fn solution() {
  let mut robots: Vec<Robot> = fs::read_to_string("src/days/day14/input.txt").unwrap().lines().map(|line| {
    let (pos, vel) = line.split_once(" ").unwrap();
    let pos = pos.split_once("=").unwrap().1.split_once(',').unwrap();
    let vel = vel.split_once("=").unwrap().1.split_once(',').unwrap();
    let pos: (i32, i32) = (pos.0.parse().unwrap(), pos.1.parse().unwrap());
    let vel: (i32, i32) = (vel.0.parse().unwrap(), vel.1.parse().unwrap());
    Robot {
      pos,
      vel
    }
  }).collect();

  for _ in 0..100 {
    robots.iter_mut().for_each(|robot| robot.step());
  }

  let mut result = 1;
  let quads: [(Range<i32>, Range<i32>); 4] = 
  [
    (0..BOUNDS.0/2, 0..BOUNDS.1/2),
    ((BOUNDS.0/2)+1..BOUNDS.0, (BOUNDS.1/2)+1..BOUNDS.1),
    (0..BOUNDS.0/2, (BOUNDS.1/2)+1..BOUNDS.1),
    ((BOUNDS.0/2)+1..BOUNDS.0, 0..BOUNDS.1/2)
  ];
  println!("{:?}", quads);
  for quad in quads.iter() {
    let mut safety = 0;
    for robot in robots.iter() {
      safety += if in_bounds(quad.clone(), robot.pos) { 1 } else { 0 }
    }
    result *= safety;
  }

  println!("14.1: {result}");
}