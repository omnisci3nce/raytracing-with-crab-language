use std::ops::{Add, Sub, Mul, Neg, Div};

#[derive(Debug, Default)]
pub struct Vec3 {
  x: f64,
  y: f64,
  z: f64
}

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

impl Mul for Vec3 {
  type Output = Self;

  fn mul(self, rhs: Self) -> Self {
    Self {x: self.x * rhs.x, y: self.y * rhs.y, z: self.z * rhs.z}
  }
}

impl Neg for Vec3 {
  type Output = Self;

  fn neg(self) -> Self {
    Self {x: -self.x, y: -self.y, z: -self.z }
  }
}

impl Div for Vec3 {
  type Output = Self;

  fn div(self, rhs: Self) -> Self {
    Self {x: self.x / rhs.x, y: self.y / rhs.y, z: self.z / rhs.z}
  }
}

impl Vec3 {
  fn length(&self) -> f64 {
    f64::sqrt(self.length_squared())
  }

  fn length_squared(&self) -> f64 {
    self.x * self.x + self.y * self.y + self.z * self.z
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
  #[test]
  fn length_pythag() { // triangle with sides 3 & 4, hypotenuse = length 5
    let v1 = Vec3{x:4.,y:3.,z:0.};
    let result = v1.length();
    assert_eq!(result, 5.0);
    assert_eq!(v1.length_squared(), 25.0);
  }
}
