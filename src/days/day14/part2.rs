use std::fs;
use image::{RgbImage, Rgb};

const BOUNDS: (i32, i32) = (101, 103);
type Image = [[bool; 101]; 103];

#[derive(Debug)]
struct Robot {
  pos: (i32, i32),
  vel: (i32, i32)
}

impl Robot {
  fn step(&mut self) {
    self.pos.0 = (self.pos.0 + self.vel.0).rem_euclid(BOUNDS.0);
    self.pos.1 = (self.pos.1 + self.vel.1).rem_euclid(BOUNDS.1);
  }
}

pub fn print_mage(image_data: Image, name: &str) -> std::io::Result<()> {
  // Crear una nueva imagen RGB de 4x4 píxeles
  let mut img: RgbImage = RgbImage::new(image_data[0].len() as u32, image_data.len() as u32);

  // Llenar la imagen según la matriz de datos
  for (y, row) in image_data.iter().enumerate() {
    for (x, &pixel) in row.iter().enumerate() {
      let color = if pixel {
        Rgb([255, 255, 255])  // Blanco
      } else {
        Rgb([0, 0, 0])  // Negro
      };
      img.put_pixel(x as u32, y as u32, color);
    }
  }

  let _ = img.save(name);

  Ok(())
}

fn new_image(robots: &Vec<Robot>) -> Image {
  let mut image: Image = [[false; 101]; 103];
  for robot in robots.iter() {
    image[robot.pos.1 as usize][robot.pos.0 as usize] = true;
  }
  image
}

pub fn solution() {
  let mut robots: Vec<Robot> = fs::read_to_string("src/days/day14/input.txt").unwrap().lines().map(|line| {
    let (pos, vel) = line.split_once(" ").unwrap();
    let pos = pos.split_once("=").unwrap().1.split_once(',').unwrap();
    let vel = vel.split_once("=").unwrap().1.split_once(',').unwrap();
    let pos: (i32, i32) = (pos.0.parse().unwrap(), pos.1.parse().unwrap());
    let vel: (i32, i32) = (vel.0.parse().unwrap(), vel.1.parse().unwrap());
    Robot {
      pos,
      vel
    }
  }).collect();

  for i in 0..(BOUNDS.0*BOUNDS.1) {
    let image = new_image(&robots);
    let _ = print_mage(image, format!("frame_{i}.png").as_str());
    robots.iter_mut().for_each(|robot| robot.step());
  }

  println!("14.2: 7492");
}