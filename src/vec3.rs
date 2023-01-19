use std::ops::{Add, Sub, Mul, Neg, Div};

#[derive(Debug, Default)]
pub struct Vec3 {
  pub x: f64,
  pub y: f64,
  pub z: f64
}

// aliases
pub type Color = Vec3;
pub type Point3 = Vec3;

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

  /* pair-wise multiply */
  fn mul(self, rhs: Self) -> Self {
    Self {x: self.x * rhs.x, y: self.y * rhs.y, z: self.z * rhs.z}
  }
}

impl Mul<f64> for Vec3 {
  type Output = Self;

  /* multiply by scalar */
  fn mul(self, rhs: f64) -> Self {
    Self {x: self.x * rhs, y: self.y * rhs, z: self.z * rhs}
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

  /* pair-wise divide */
  fn div(self, rhs: Self) -> Self {
    Self {x: self.x / rhs.x, y: self.y / rhs.y, z: self.z / rhs.z}
  }
}

impl Div<f64> for Vec3 {
  type Output = Self;

  /* divide by scalar */
  fn div(self, rhs: f64) -> Self {
    self * (1.0/rhs)
  }
}

impl Vec3 {
  fn length(&self) -> f64 {
    f64::sqrt(self.length_squared())
  }

  fn length_squared(&self) -> f64 {
    self.x * self.x + self.y * self.y + self.z * self.z
  }

  /* dot product */
  fn dot(u: &Self, v: &Self) -> f64 {
    u.x * v.x + u.y * v.y + u.z * v.z
  }

  /* cross product */
  fn cross(u: &Self, v: &Self) -> Vec3 {
    Vec3 {
      x: u.y * v.z - u.z * v.y,
      y: u.z * v.x - u.x * v.z,
      z: u.x * v.y - u.y * v.x
    }
  }
  
  fn unit_vector(self) -> Vec3 {
    //self / self.length()
    let k = 1.0 / self.length();
    self * k
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

  #[test]
  fn unit_vector_should_have_length_1() {
    let u = Vec3{x:10.5,y:100.0,z:-4.0};
    assert_eq!(f64::round(u.unit_vector().length()), 1.0);
  }
}
