use std::{collections::BTreeMap, fs};

pub fn solution() {
  let input = fs::read_to_string("src/days/day09/input.txt").unwrap();
  let mut disk: Vec<String> = Vec::new();
  let mut free: Vec<(usize, usize)> = Vec::new(); 
  let mut blocks: BTreeMap<String, (usize, usize)> = BTreeMap::new();
  for (i, c) in input.chars().enumerate() {
    let c: usize = (c as usize) - ('0' as usize);
    if i % 2 == 0 {
      let i = i/2;
      disk.extend(vec![i.to_string(); c]);
    }
    else {
      disk.extend(vec![".".to_string(); c]);
    }
  }

  let mut dot = false;
  let mut aux = 0;
  disk.iter().enumerate().for_each(|(i, c)| {
    if c != "." {
      blocks.entry(c.clone()).or_insert((i, i)).1 = i;
      dot = false;
    }
    else {
      if !dot {
        free.push((i, i));
        aux += 1;
      }
      else {
        free[aux - 1].1 = i;
      }
      dot = true;
    }
  });
  order(&mut disk, &mut free, &mut blocks);
  let mut result = 0;

  disk.iter().enumerate().for_each(|(i, c)| {
    let c: usize = c.parse().unwrap_or(0);
    result += i * c;
  });

  println!("9.2: {result}");
}

fn order(disk: &mut Vec<String>, free: &mut Vec<(usize, usize)>, blocks: &mut BTreeMap<String, (usize, usize)>) {

  for (c, i) in blocks.iter().rev() {
    for x in 0..free.len() {
      let block_size = (i.1 - i.0) + 1;
      let free_size = (free[x].1 - free[x].0) + 1;
      if free_size >= block_size && i.1 > free[x].0 {

        for i in free[x].0..(free[x].0 + block_size) {
          disk[i] = c.clone();
        }
        for i in i.0..=i.1 {
          disk[i] = ".".to_string();
        }
        free[x].0 += block_size;
        if free[x].0 > free[x].1 {
          free.remove(x);
        }
        break;
      }
    }
  }
}