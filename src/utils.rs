use crate::vec3::Color;

pub fn write_color(c: Color) {
  let ir = (255.999 * c.x) as i32;
  let ig = (255.999 * c.y) as i32;
  let ib = (255.999 * c.z) as i32;
  println!("{} {} {}", ir, ig, ib);
}
