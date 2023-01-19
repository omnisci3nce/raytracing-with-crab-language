mod vec3;
const IMAGE_WIDTH : i32 = 256;
const IMAGE_HEIGHT : i32 = 256;

fn main() {
  println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);
  for j in (0..IMAGE_HEIGHT).rev() {
    eprintln!("\rScanlines remaining: {}", j);
    for i in 0..IMAGE_WIDTH {
      let r = i as f64 / (IMAGE_WIDTH as f64 - 1.0);
      let g = j as f64 / (IMAGE_HEIGHT as f64 - 1.0);
      let b = 0.25;

      let ir = (255.999 * r) as i32;
      let ig = (255.999 * g) as i32;
      let ib = (255.999 * b) as i32;
      println!("{} {} {}", ir, ig, ib);
    }
  }

  println!("Done.");
}
