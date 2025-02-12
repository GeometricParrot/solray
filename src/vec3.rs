use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign, Neg};
use std::ops::Range;
pub use rand::{Rng, SeedableRng};
pub use rand_chacha::ChaCha8Rng;

#[derive(Debug, Copy, Clone)]
pub struct Vec3{
	pub x: f32,
	pub y: f32,
	pub z: f32,
}

// unary opereators
impl Neg for Vec3 {
	type Output = Vec3;

	fn neg(self) -> Vec3{
		Vec3 {
			x: -self.x,
			y: -self.y,
			z: -self.z,
		}
	}
}

// binary opereators
impl Add for Vec3 {
	type Output = Vec3;

	fn add(self, other: Vec3) -> Vec3{
		Vec3 {
			x: self.x + other.x,
			y: self.y + other.y,
			z: self.z + other.z,
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
impl Mul<Vec3> for f32 {
	type Output = Vec3;

	fn mul(self, other: Vec3) -> Vec3{
		Vec3 {
			x: self * other.x,
			y: self * other.y,
			z: self * other.z,
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
impl Div<Vec3> for f32 {
	type Output = Vec3;

	fn div(self, other: Vec3) -> Vec3{
		Vec3 {
			x: self / other.x,
			y: self / other.y,
			z: self / other.z,
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
	pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
		Vec3{x: x, y: y, z: z}
	}
	pub fn length_squared(&self) -> f32 {
		self.x * self.x + self.y*self.y + self.z*self.z
	}
	pub fn length(&self) -> f32 {
		self.length_squared().sqrt()
	}
	pub fn dot(&self, other: &Vec3) -> f32{
		self.x * other.x
		+ self.y * other.y
		+ self.z * other.z
	}
	pub fn cross(&self, other: &Vec3) -> Vec3 {
		Vec3 {
			x: self.y * other.z - self.z * other.y,
			y: self.z * other.x - self.x * other.z,
			z: self.x * other.y - self.y * other.x,
		}
	}
	pub fn normalized(&self) -> Vec3 {
		*self / self.length()
	}
	pub fn random(range: Range<f32>, rng: &mut ChaCha8Rng) -> Vec3 {
		Vec3::new(rng.random_range(range.clone()), rng.random_range(range.clone()), rng.random_range(range.clone()))
	}
	pub fn random_unit_vector(rng: &mut ChaCha8Rng) -> Vec3 {
		loop {
			let p = Vec3::random(-1.0..1.0, rng);
			let length_squared = p.length_squared();
			if 1e-40 < length_squared && length_squared <= 1.0 {
				return p / length_squared.sqrt();
			}
		}
	}
	pub fn random_on_hemisphere(normal: &Vec3, rng: &mut ChaCha8Rng) -> Vec3 {
		let on_unit_sphere = Vec3::random_unit_vector(rng);
		if Vec3::dot(&on_unit_sphere, normal) > 0.0 {
			return on_unit_sphere;
		}
		else {
			return -on_unit_sphere;
		}
	}
}

pub use Vec3 as Point3;

pub use Vec3 as Color;

pub fn linear_to_gamma(linear_component: f32) -> f32 {
	if linear_component > 0.0 {
		return linear_component.sqrt();
	}
	return 0.0;
}

pub fn write_color(color: &Color) {
	// Translate the [0,1] component values to the byte range [0,255].
	let r = (linear_to_gamma(color.x).clamp(0.0, 1.0) * 255.999) as i32;
	let g = (linear_to_gamma(color.y).clamp(0.0, 1.0) * 255.999) as i32;
	let b = (linear_to_gamma(color.z).clamp(0.0, 1.0) * 255.999) as i32;

	println!("{} {} {}", r, g, b);
}