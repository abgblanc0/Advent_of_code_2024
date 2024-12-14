use std::fs;

type Label = [[i32; 3]; 2];


pub fn solution() {
  let input = fs::read_to_string("src/days/day13/input.txt").unwrap().replace("\r\n", "\n");
  
  let mut machines: Vec<Vec<Label>> = Vec::new();
  input.split("\n\n").for_each(|m| {
    let mac: String = m.chars().filter(|c| c.is_numeric() || "+=XY".contains(*c)).collect();
    machines.push(mac.lines().map(|line| { 
      let nums: Vec<i32> = line.split(|c: char| !c.is_numeric()).filter(|s| !s.is_empty()).map(|s| s.parse().unwrap()).collect();

      [ [nums[0], nums[2], nums[4]], [nums[1], nums[3], nums[5]] ]
    }).collect());
  });

  let mut result = 0;

  for machine in machines {
    for label in machine {
      let det = (label[0][0] * label[1][1]) - (label[1][0] * label[0][1]);
      let det_a = (label[0][2] * label[1][1]) - (label[1][2] * label[0][1]);
      let det_b = (label[0][0] * label[1][2]) - (label[1][0] * label[0][2]);
      if det == 0 || det_a%det != 0 || det_b%det != 0 { continue; }
      let a = det_a / det;
      let b = det_b / det;

      result += 3*a + b;
    }
  }
  
  println!("13.1: {result}");
}