use std::fs;

fn get_char(matrix: &mut Vec<Vec<char>>, index: (usize, usize)) -> char {
  *matrix.get(index.0).unwrap_or(&vec!['.']).get(index.1).unwrap_or(&'.')
}

fn check(matrix: &mut Vec<Vec<char>>, index: (usize, usize)) -> i32 {
  let index: (i32, i32) = (index.0 as i32, index.1 as i32);
  let directions: [(i32, i32); 8] = [
    (0, 1),   // Derecha
    (0, -1),  // Izquierda
    (1, 0),   // Abajo
    (-1, 0),  // Arriba
    (1, 1),   // Diagonal abajo-derecha
    (1, -1),  // Diagonal abajo-izquierda
    (-1, 1),  // Diagonal arriba-derecha
    (-1, -1), // Diagonal arriba-izquierda
  ];

  let target = ['X', 'M', 'A', 'S'];
  let mut result = 0;

  for (dx, dy) in directions {
    let mut ok = true;
    for (step, &char_target) in target.iter().enumerate() {
      let x = index.0 + step as i32 * dx;
      let y = index.1 + step as i32 * dy;
      if get_char(matrix, (x as usize, y as usize)) != char_target {
        ok = false;
        break;
      }
    }
    if ok {
      result += 1;
    }
  }

  result
}


pub fn solution() {
  let input = fs::read_to_string("src/days/day04/input.txt").unwrap();
  let mut result = 0;
  let mut matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

  let rows = matrix.len();
  let cols = matrix[0].len();
  for index_y in 0..rows {
    for index_x in 0..cols {
        match matrix[index_y][index_x] {
            'X' => result += check(&mut matrix, (index_y, index_x)),
            _ => {}
        }
    }
  }

  println!("4.1: {}", result);
}
