use std::fs;

const DIRECTIONS: [(char, (isize, isize)); 4] = [
    ('^', (-1, 0)), // Arriba
    ('>', (0, 1)),  // Derecha
    ('v', (1, 0)),  // Abajo
    ('<', (0, -1)), // Izquierda
];
/* 
fn print_matrix(matrix: &Vec<Vec<char>>) {
  for row in matrix {
      println!("{}", row.iter().collect::<String>());
  }
}
*/
fn turn_right(matrix: &mut Vec<Vec<char>>, position: &mut (usize, usize)) {
  if let Some(index) = DIRECTIONS.iter().position(|&dir| dir.0 == matrix[position.0][position.1]) {
      let next_index = (index + 1) % DIRECTIONS.len();
      matrix[position.0][position.1] = DIRECTIONS[next_index].0;
  }
}

fn watch_front(matrix: &Vec<Vec<char>>, position: (usize, usize), direction: char) -> char {
  // Busca el desplazamiento asociado con la dirección actual.
  if let Some(&(_, (dx, dy))) = DIRECTIONS.iter().find(|&&(dir, _)| dir == direction) {
    let new_x = position.0 as isize + dx * 2;
    let new_y = position.1 as isize + dy * 2;
    let c = *matrix.get(new_x as usize).unwrap_or(&vec!['!']).get(new_y as usize).unwrap_or(&'!');
    return c;
  }
  '!'
}

fn is_loop(matrix: &Vec<Vec<char>>, position: (usize, usize), direction: char) -> bool {
  let mut old_pos: Vec<(usize, usize)> = Vec::new();
  let mut current_position = position;
  let mut current_direction = direction;

  loop {
    let right_direction = match current_direction {
      '^' => '>',
      '>' => 'v',
      'v' => '<',
      '<' => '^',
      _ => return false,
    };

    if let Some(&(_, (dx, dy))) = DIRECTIONS.iter().find(|&&(dir, _)| dir == right_direction) {
      let new_pos = (
        (current_position.0 as isize + dx) as usize,
        (current_position.1 as isize + dy) as usize,
      );

      let c = *matrix
        .get(new_pos.0)
        .unwrap_or(&vec!['!'])
        .get(new_pos.1)
        .unwrap_or(&'!');

      let front = watch_front(matrix, current_position, right_direction);
      
      if front == '#' {
        current_position = new_pos;
        current_direction = right_direction;
        continue;
      }
      if front == '!' || c == '!' {
        return false;
      }
      if DIRECTIONS.iter().any(|(dir, _)| dir == &c) {
        return true;
      }
      //print!("{:?}: {c}", current_position);
      current_position = new_pos;
      if old_pos.contains(&current_position) {
        return true;
      }
      else {
        old_pos.push(current_position);
      }
    } else {
      return false;
    }
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

  let mut result = 0;
  while matrix[position.0][position.1] != 'X' {
    let direction = matrix[position.0][position.1];
    if !push(&mut matrix, &mut position) {
      turn_right(&mut matrix, &mut position);
    }
    if is_loop(&matrix, position, direction) {
      result +=1;
    };
  }

  println!("6.2: {result}");
}