use image::{RgbImage, Rgb};
type Image = [[bool; 101]; 103];

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