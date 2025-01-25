use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Debug, Copy, Clone)]
pub struct Vec3{
	pub x: f32,
	pub y: f32,
	pub z: f32,
}

// binary opereators
impl Add for Vec3 {
	type Output = Vec3;

	fn add(self, other: Vec3) -> Vec3{
		Vec3 {
			x: self.x - other.x,
			y: self.y - other.y,
			z: self.z - other.z,
		}
	}
}
impl Sub for Vec3 {
	type Output = Vec3;

	fn sub(self, other: Vec3) -> Vec3{
		Vec3 {
			x: self.x - other.x,
			y: self.y - other.y,
			z: self.z - other.z,
		}
	}
}
impl Mul<Vec3> for Vec3 {
	type Output = Vec3;

	fn mul(self, other: Vec3) -> Vec3{
		Vec3 {
			x: self.x * other.x,
			y: self.y * other.y,
			z: self.z * other.z,
		}
	}
}
impl Mul<f32> for Vec3 {
	type Output = Vec3;

	fn mul(self, factor: f32) -> Vec3{
		Vec3 {
			x: self.x * factor,
			y: self.y * factor,
			z: self.z * factor,
		}
	}
}
impl Div<Vec3> for Vec3 {
	type Output = Vec3;

	fn div(self, other: Vec3) -> Vec3{
		Vec3 {
			x: self.x / other.x,
			y: self.y / other.y,
			z: self.z / other.z,
		}
	}
}
impl Div<f32> for Vec3 {
	type Output = Vec3;

	fn div(self, factor: f32) -> Vec3{
		Vec3 {
			x: self.x / factor,
			y: self.y / factor,
			z: self.z / factor,
		}
	}
}

// assignment operators
impl AddAssign for Vec3 {
	fn add_assign(&mut self, rhs: Vec3) {
		self.x += rhs.x;
		self.y += rhs.y;
		self.z += rhs.z;
	}
}
impl SubAssign for Vec3 {
	fn sub_assign(&mut self, rhs: Vec3) {
		self.x -= rhs.x;
		self.y -= rhs.y;
		self.z -= rhs.z;
	}
}
impl MulAssign<Vec3> for Vec3 {
	fn mul_assign(&mut self, rhs: Vec3) {
		self.x *= rhs.x;
		self.y *= rhs.y;
		self.z *= rhs.z;
	}
}
impl MulAssign<f32> for Vec3 {
	fn mul_assign(&mut self, rhs: f32) {
		self.x *= rhs;
		self.y *= rhs;
		self.z *= rhs;
	}
}
impl DivAssign<Vec3> for Vec3 {
	fn div_assign(&mut self, rhs: Vec3) {
		self.x /= rhs.x;
		self.y /= rhs.y;
		self.z /= rhs.z;
	}
}
impl DivAssign<f32> for Vec3 {
	fn div_assign(&mut self, rhs: f32) {
		self.x /= rhs;
		self.y /= rhs;
		self.z /= rhs;
	}
}

impl Vec3 {
	fn length_squared(&self) -> f32 {
		self.x * self.x + self.y*self.y + self.z*self.z
	}
	fn length(&self) -> f32 {
		self.length_squared().sqrt()
	}
	fn dot(&self, other: &Vec3) -> f32{
		self.x * other.x
		+ self.y * other.y
		+ self.z * other.z
	}
	fn cross(&self, other: &Vec3) -> Vec3 {
		Vec3 {
			x: self.y * other.z - self.z * other.y,
			y: self.z * other.x - self.x * other.z,
			z: self.x * other.y - self.y * other.x,
		}
	}
	fn normalized(&self) -> Vec3 {
		*self / self.length()
	}
}

pub use Vec3 as Point3;

pub use Vec3 as Color;

pub fn write_color(color: &Color) {
	// Translate the [0,1] component values to the byte range [0,255].
	let r = (color.x * 255.999) as i32;
	let g = (color.y * 255.999) as i32;
	let b = (color.z * 255.999) as i32;

	println!("{} {} {}", r, g, b);
}