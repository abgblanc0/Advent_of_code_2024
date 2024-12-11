use std::fs;

pub fn solution() {
  let input = fs::read_to_string("src/days/day09/input.txt").unwrap();
  let mut disk: Vec<String> = Vec::new();
  let mut free: Vec<usize> = Vec::new(); 

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

  disk.iter().enumerate().filter(|(_,c)| c == &&".").for_each(|(i, _)| {
    free.push(i);
  });

  order(&mut disk, &mut free);
  let mut result = 0;
  disk.iter().filter(|c| c != &&".").enumerate().for_each(|(i, c)| {
    let c: usize = c.parse().unwrap();
    result += i * c;
  });
  println!("9.1: {result}");
}

fn order(disk: &mut Vec<String>, free: &mut Vec<usize>) {
  for i in (0..disk.len()).rev() {
    if disk[i] != "." {
      if let Some(&first_free) = free.first() {
        disk[first_free] = disk[i].clone();
        free.remove(0);
        disk[i] = ".".to_string();
      }
    }
    if disk[..i].iter().rev().all(|c| *c != ".") { break; }
  }
}