use std::{collections::HashMap, fs};

fn check(update: &Vec<i32>, rules: &HashMap<i32, Vec<i32>>) -> bool {
  let mut ok = false;
  for (index, num) in update.iter().enumerate() {
    //println!("{:?} {num}: {:?} || {:?}", update.split_at(index).1, rules.get(num), update.split_at(index).1.iter().any(|aux| { rules.get(num).iter().any(|rule| rule.contains(aux))}));
    ok = !update.split_at(index).1.iter().any(|aux| { rules.get(num).iter().any(|rule| rule.contains(aux))});
    if !ok { break; }
  }
  ok
}

fn get_rules(rules: &str) -> HashMap<i32, Vec<i32>> {
  let mut associations: HashMap<i32, Vec<i32>> = HashMap::new();
  for line in rules.lines() {
    let (left, right) = line.split_once('|').unwrap();
    let left: i32 = left.parse().unwrap();
    let right: i32 = right.parse().unwrap();

    associations
      .entry(right) 
      .or_insert_with(Vec::new)
      .push(left);
  }
  return associations;
}
pub fn solution() {
  let input = fs::read_to_string("src/days/day05/input.txt").unwrap().replace("\r\n", "\n");
  let (rules, updates) = input.split_once("\n\n").unwrap();
  let rules: HashMap<i32, Vec<i32>> = get_rules(rules);
  let mut ok_ups:Vec<Vec<i32>> = Vec::new();
  let mut result = 0;
  let updates: Vec<Vec<i32>> = updates.lines().map(|line|
      line.split(',')
          .map(|num| num.parse().unwrap())
          .collect::<Vec<i32>>()
  ).collect();

  for update in updates {
    if check(&update, &rules) { ok_ups.push(update); }
  }
  ok_ups.iter().for_each(|o| result += o.get(o.len() / 2).unwrap_or(&0));
  println!("5.1: {result}");
}