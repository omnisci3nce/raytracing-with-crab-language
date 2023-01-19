use std::ops::{Add, Sub};

#[derive(Debug, Default)]
pub struct Vec3 {
  x: f64,
  y: f64,
  z: f64
}

//impl Vec3 {

// Operator Overloading: https://doc.rust-lang.org/rust-by-example/trait/ops.html

impl Add for Vec3 {
  type Output = Self;

  fn add(self, rhs: Self) -> Self { // rhs = right-hand side
    Self {x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z}
  }
}

impl Sub for Vec3 {
  type Output = Self;

  fn sub(self, rhs: Self) -> Self { // rhs = right-hand side
    Self {x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z}
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn add_vec3() {
    let v1 = Vec3{ x: 5., y: 5., z: 10.};
    let v2 = Vec3{ x: 5., y: 3., z: 10.};
    let result = v1 + v2;
    assert_eq!(result.x, 10.0);
    assert_eq!(result.y, 8.0);
    assert_eq!(result.z, 20.0);
  }
  #[test]
  fn sub_vec3() {
    let v1 = Vec3{ x: 5., y: 5., z: 10.};
    let v2 = Vec3{ x: 5., y: 3., z: 10.};
    let result = v1 - v2;
    assert_eq!(result.x, 0.0);
    assert_eq!(result.y, 2.0);
    assert_eq!(result.z, 0.0);
  }
}
