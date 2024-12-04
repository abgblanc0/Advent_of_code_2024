use std::fs;

fn get_char(matrix: &mut Vec<Vec<char>>, index: (usize, usize)) -> char {
  *matrix.get(index.0).unwrap_or(&vec!['.']).get(index.1).unwrap_or(&'.')
}

fn check(matrix: &mut Vec<Vec<char>>, index: (usize, usize)) -> bool {
  let a = get_char(matrix, ((index.0 as i32 - 1) as usize, (index.1 as i32 - 1) as usize));
  let d = get_char(matrix, (index.0 + 1, index.1 + 1));
  let b = get_char(matrix, ((index.0 as i32 - 1) as usize, index.1 + 1));
  let c = get_char(matrix, (index.0 + 1, (index.1 as i32 - 1) as usize));
  return (a == 'M' || a == 'S') && (b == 'M' || b == 'S') && (c == 'M' || c == 'S') && (d == 'M' || d == 'S') && a != d && b != c;
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
            'A' => result += if check(&mut matrix, (index_y, index_x)) {1} else {0},
            _ => {}
        }
    }
  }

  println!("4.2: {}", result);
}
