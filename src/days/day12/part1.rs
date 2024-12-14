use std::fs;


type Plot = Vec<Vec<char>>;
type Coord = (usize, usize);

#[derive(Clone, Debug)]
struct Area {
  matrix: Vec<Vec<Coord>>,
  value: u32,
}


fn get(garden: &Plot, pos: Coord) -> char {
  if let Some(line) = garden.get(pos.0) {
    return *line.get(pos.1).unwrap_or(&'.');
  }
  '.'
}

fn check_slice(garden: &mut Plot, slice: (usize, usize), x: usize, plant: char) -> bool {
  if let Some(line) = garden.get_mut(x) {
    if line[slice.0..=slice.1].iter().all(|c| *c == plant) {
      line[slice.0..=slice.1].iter_mut().for_each(|c|  *c = '.');
      return true;
    }
  }
  false
}

pub fn solution() {
  let mut garden: Plot = fs::read_to_string("src/days/day12/input.txt").unwrap().lines().map(|line| line.chars().collect()).collect();
  
  let mut areas: Vec<(char, Area)> = Vec::new();

  let mut x = 0;
  let mut y = 0;
  let mut prev;
  let mut next;

  let mut current_area: Area = Area {
    matrix: Vec::new(),
    value: 0,
  };
  let mut aux: Vec<Coord> = Vec::new();
  
  while x < garden.len() {
    while y < garden[x].len() {
      let plant = garden[x][y];
      if plant == '.' { y += 1; continue; }
      
      prev = get(&garden, (x, y.wrapping_sub(1))) == plant;
      next = get(&garden, (x, y + 1)) == plant;

      if !prev {
        current_area = Area {
          matrix: Vec::new(),
          value: 1,
        };
        aux = vec![(x,y)];
      }
      else {
        aux.push((x,y));
        current_area.value += 1;
      }
      if !next {
        let slice = (aux.first().unwrap().1, aux.last().unwrap().1);
        garden[x][slice.0..=slice.1].iter_mut().for_each(|c|  *c = '.');
        let mut x = x + 1;
        current_area.matrix.push(aux.clone());
        
        while check_slice(&mut garden, slice, x, plant) {
          aux = vec![];
          x += 1;
          for i in slice.0..=slice.1 {
            aux.push((x, i));
            current_area.value += 1;
          }
          current_area.matrix.push(aux.clone());
        }

        areas.push((plant, current_area.clone()));
      }
      y += 1;
    }
    y = 0;
    x += 1;
  }
  areas.iter().for_each(|area| {
    println!("{:?}", area)
  });
}