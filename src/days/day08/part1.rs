use std::{collections::{HashMap, HashSet}, fs};

pub fn solution() {
  let mut input: Vec<Vec<char>> = fs::read_to_string("src/days/day08/input.txt").unwrap().lines().map(|line| line.chars().map(|c| c).collect()).collect();
  let mut antennas: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
  
  for x in 0..input.len() {
    for y in 0..input[x].len() {
      let c = input[x][y];
      if c != '.' {
        antennas.entry(c).or_insert_with(Vec::new).push((x,y));
      }
    }
  }
  let mut result: HashSet<(usize, usize)> = HashSet::new();
  antennas.iter().for_each(|(c, coords)| {
    result.extend(antinodes(coords, &mut input, *c));
  });
  println!("8.1: {}", result.len());
}

fn antinodes(coords: &Vec<(usize, usize)>, input: &mut Vec<Vec<char>>, c: char) -> HashSet<(usize, usize)> {
  let mut result: HashSet<(usize, usize)> = HashSet::new();
  for x in 0..coords.len() {
    for y in x+1..coords.len() {
      let diff = (coords[x].0.abs_diff(coords[y].0), coords[x].1.abs_diff(coords[y].1));

      let (xx, yy) = (if coords[x].0 < coords[y].0 { coords[x].0.wrapping_sub(diff.0) } else { coords[x].0 + diff.0 }, if coords[x].1 < coords[y].1 { coords[x].1.wrapping_sub(diff.1) } else { coords[x].1 + diff.1 });
      let x_c = *input.get(xx).unwrap_or(&vec!['!']).get(yy).unwrap_or(&'!');
      if x_c != '!' && x_c != c {
        result.insert((xx, yy));
      }

      let (xx, yy) = (if coords[y].0 < coords[x].0 { coords[y].0.wrapping_sub(diff.0) } else { coords[y].0 + diff.0 }, if coords[y].1 < coords[x].1 { coords[y].1.wrapping_sub(diff.1) } else { coords[y].1 + diff.1 });
      let y_c = *input.get(xx).unwrap_or(&vec!['!']).get(yy).unwrap_or(&'!');
      if y_c != '!' && y_c != c {
        result.insert((xx,yy));
      }
    }
  }
  result
}