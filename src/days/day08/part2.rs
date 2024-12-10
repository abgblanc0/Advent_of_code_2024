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
  antennas.iter().for_each(|(_ ,coords)| {
    result.extend(antinodes(coords, &mut input));
  });
  result.iter().for_each(|coord| {
    let c = input[coord.0][coord.1];
    if c != '0' && c != 'A' {
      input[coord.0][coord.1] = '#';
    }
  });
  println!("8.2: {}", result.len());
}


fn minus(input: &mut Vec<Vec<char>> ,coord: (usize, usize), diff: (isize, isize)) -> HashSet<(usize, usize)> {
  let mut result: HashSet<(usize,usize)> = HashSet::new();

  let c = *input.get(coord.0).unwrap_or(&vec!['!']).get(coord.1).unwrap_or(&'!');
  if c != '!' { result.insert(coord); } else { return result }
  let coord: (usize, usize) = ((coord.0 as isize - diff.0) as usize, (coord.1 as isize - diff.1) as usize);
  
  result.extend(minus(input, coord, diff));
  result
}

fn plus(input: &mut Vec<Vec<char>> ,coord: (usize, usize), diff: (isize, isize)) -> HashSet<(usize, usize)> {
  let mut result: HashSet<(usize,usize)> = HashSet::new();
  
  let c = *input.get(coord.0).unwrap_or(&vec!['!']).get(coord.1).unwrap_or(&'!');
  if c != '!' { result.insert(coord); } else { return result }
  let coord: (usize, usize) = ((coord.0 as isize + diff.0) as usize, (coord.1 as isize + diff.1) as usize);
  result.extend(plus(input, coord, diff));
  result
}

fn antinodes(coords: &Vec<(usize, usize)>, input: &mut Vec<Vec<char>>) -> HashSet<(usize, usize)> {
  let mut result: HashSet<(usize, usize)> = HashSet::new();
  for x in 0..coords.len() {
    for y in x+1..coords.len() {
      let diff = (coords[x].0 as isize - coords[y].0 as isize, coords[x].1 as isize - coords[y].1 as isize);
      
      result.extend(minus(input, coords[y], diff));
      result.extend(plus(input, coords[y], diff));
    }
  }
  result
}