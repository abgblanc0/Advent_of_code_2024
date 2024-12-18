use std::fs;

#[derive(Debug, Clone)]
struct Game {
  map: Vec<Vec<char>>,
  pos: (usize, usize),
  moves: Vec<(isize, isize)>,
}

impl Game {
  fn new(input: String) -> Game {
    let (map, moves) = input.split_once("\n\n").unwrap();
    let map: Vec<Vec<char>> = map.lines().map(|line| line.chars().collect()).collect();
    let mut pos: (usize, usize) = (0, 0);

    for (x, line) in map.iter().enumerate() {
      for (y, c) in line.iter().enumerate() {
        if *c == '@' { pos = (x, y); }
      }
    }

    let moves: Vec<(isize, isize)> = moves.lines().collect::<String>().chars().map(|c| {
      match c {
        '^' => (-1, 0),
        '>' => (0, 1),
        '<' => (0, -1),
        'v' => (1, 0),
        _ => unreachable!(),
      }
    }).collect();

    Game { map, pos, moves }
  }

  fn print(&self) {
    for line in self.map.iter() {
      for c in line {
        print!("{c}")
      }
      println!()
    }
  }

  fn play(&mut self) {
    for i in 0..self.moves.len() {
      let dir = self.moves[i];
      let new_pos = ((self.pos.0 as isize + dir.0) as usize, (self.pos.1 as isize + dir.1) as usize);
      let c = self.get(new_pos);
      if c == '#' { continue; }
      if (c == '.') || (c == 'O' && self.push(new_pos, dir)) { 
        self.map[self.pos.0][self.pos.1] = '.';
        self.map[new_pos.0][new_pos.1] = '@';
        self.pos = new_pos;
      }
    }
  }

  fn push(&mut self, pos: (usize, usize), dir: (isize, isize)) -> bool {
    let new_pos = ((pos.0 as isize + dir.0) as usize, (pos.1 as isize + dir.1) as usize);
    let c = self.get(new_pos);
    if c == 'O' {
      if self.push(new_pos, dir) {
        self.map[pos.0][pos.1] = '.';
        self.map[new_pos.0][new_pos.1] = 'O';
        return true;
      }
    }
    if c == '.' {
      self.map[new_pos.0][new_pos.1] = 'O';
      self.map[pos.0][pos.1] = '.';
      return true
    }
    false
  }

  fn get(&self, pos: (usize, usize)) -> char {
    if let Some(line) = self.map.get(pos.0) {
      return *line.get(pos.1).unwrap_or(&'#')
    }
    '#'
  }

}

pub fn solution() {
  let input = fs::read_to_string("src/days/day15/input.txt").unwrap().replace("\r\n", "\n");
  let mut game = Game::new(input);

  game.play();

  let mut result = 0;

  for (y, line) in game.map.iter().enumerate() {
    for (x, c) in line.iter().enumerate() {
      if *c == 'O' {
        result += (y * 100) + x;
      }
    }
  }

  println!("15.1: {result}");
}