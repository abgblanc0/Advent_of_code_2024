use std::fs;

const DIRECTIONS: [(char, (isize, isize)); 4] = [
    ('^', (-1, 0)), // Arriba
    ('>', (0, 1)),  // Derecha
    ('v', (1, 0)),  // Abajo
    ('<', (0, -1)), // Izquierda
];

fn turn_right(matrix: &mut Vec<Vec<char>>, position: &mut (usize, usize)) {
  if let Some(index) = DIRECTIONS.iter().position(|&dir| dir.0 == matrix[position.0][position.1]) {
      let next_index = (index + 1) % DIRECTIONS.len();
      matrix[position.0][position.1] = DIRECTIONS[next_index].0;
  }
}
fn push(matrix: &mut Vec<Vec<char>>, position: &mut (usize, usize)) -> bool {

  if let Some(&(dir, (dx, dy))) = DIRECTIONS.iter().find(|&&(dir, _)| dir == matrix[position.0][position.1]) {
    let new_x = position.0 as isize + dx;
    let new_y = position.1 as isize + dy;
    let next = *matrix.get(new_x as usize).unwrap_or(&vec!['!']).get(new_y as usize).unwrap_or(&'!');
    if next == '#' {
      return false;
    }
    if next == '!' {
      matrix[position.0][position.1] = 'X';
      return true;
    }
    matrix[position.0][position.1] = 'X';
    matrix[new_x as usize][new_y as usize] = dir;
  
    position.0 = new_x as usize;
    position.1 = new_y as usize;
  }
  return true;
}

pub fn solution() {
  let input = fs::read_to_string("src/days/day06/input.txt").unwrap();
  let mut matrix: Vec<Vec<char>> = input.lines().map(|line: &str| line.chars().collect()).collect();
  let mut position: (usize, usize) = (0,0);
  matrix.iter().enumerate().for_each(|(x, line)| line.iter().enumerate().for_each(|(y, c)| if c.eq(&'^') {position = (x,y)}));

  while matrix[position.0][position.1] != 'X' {
    if !push(&mut matrix, &mut position) {
      turn_right(&mut matrix, &mut position);
    }
  }

  let mut result = 0;
  matrix.iter().for_each(|line| line.iter().for_each(|&c| result += if c == 'X' {1} else {0}));
  println!("6.1: {result}");
}