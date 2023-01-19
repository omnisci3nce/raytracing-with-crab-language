mod vec3;
mod utils;
use crate::vec3::Color;
use crate::utils::write_color;
const IMAGE_WIDTH : i32 = 256;
const IMAGE_HEIGHT : i32 = 256;

fn main() {
  println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);
  for j in (0..IMAGE_HEIGHT).rev() {
    eprintln!("\rScanlines remaining: {}", j);
    for i in 0..IMAGE_WIDTH {
      let pixel_color = Color{
        x: i as f64 / (IMAGE_WIDTH as f64 - 1.0),
        y: j as f64 / (IMAGE_HEIGHT as f64 - 1.0),
        z: 0.25
      };
      write_color(pixel_color);
    }
  }

  eprintln!("Done.");
}
