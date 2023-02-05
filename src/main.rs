mod vec3;
mod utils;
mod ray;

use crate::vec3::{Vec3, Point3, Color};
use crate::utils::write_color;
use crate::ray::Ray;

const ASPECT_RATIO : f64 = 16.0 / 9.0;
const IMAGE_WIDTH : i32 = 400;
const IMAGE_HEIGHT : i32 =(IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;

fn hit_sphere(center: &Point3, radius: f64, r: &Ray) -> f64 {
  let oc = r.origin - *center;
  let a = Vec3::dot(&r.direction, &r.direction);
  let b = Vec3::dot(&oc, &r.direction) * 2.0;
  let c = Vec3::dot(&oc, &oc) - (radius * radius);
  let discriminant = b * b -  (a * c) * 4.0;
  if discriminant < 0.0 {
    -1.0
  } else {
    (-b - discriminant.sqrt()) / (a * 2.0)
  }
}

fn ray_color(r: &Ray) -> Color {
  let t = hit_sphere(&Point3{x:0.0, y:0.0, z:-1.0}, 0.5, r);
  if t > 0.0 {
    let N = (r.at(t) - Vec3{x: 0.0, y:0.0, z:-1.0}).unit_vector();
    return Color{x:N.x + 1.0,y: N.y + 1., z: N.z + 1.} * 0.5;
  }
  let unit_direction = r.direction.unit_vector();
  let t = 0.5 * (unit_direction.y + 1.0);
  Color{x: 1.0, y:1.0,z:1.0} * (1.0 - t) + Color{x:0.5,y:0.7,z:1.0} * t
}

fn main() {
  // Camera
  let viewport_height = 2.0;
  let viewport_width = ASPECT_RATIO * viewport_height;
  let focal_length = 1.0;

  let origin = Point3{x:0.0,y:0.0,z:0.0};
  let horizontal = Vec3{x:viewport_width,y:0.0,z:0.0};
  let vertical = Vec3{x:0.0,y:viewport_height,z:0.0};
  let lower_left_corner = origin - horizontal/2. - vertical/2. - Vec3{x:0.0,y:0.0,z:focal_length};

  println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);
  for j in (0..IMAGE_HEIGHT).rev() {
    eprintln!("\rScanlines remaining: {}", j);
    for i in 0..IMAGE_WIDTH {
      let u = i as f64 / (IMAGE_WIDTH as f64 - 1.0);
      let v = j as f64 / (IMAGE_HEIGHT as f64 - 1.0);
      let r = Ray{origin:origin, direction: lower_left_corner + horizontal*u + vertical*v - origin};
      let pixel_color = ray_color(&r);
      write_color(pixel_color);
    }
  }

  eprintln!("Done.");
}
