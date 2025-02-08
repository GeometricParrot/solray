pub use crate::vec3::*;

#[derive(Debug, Copy, Clone)]
pub struct Ray {
	pub origin: Point3,
	pub dir: Vec3,
}

impl Ray {
	pub fn new(origin: Point3, dir: Vec3) -> Ray {
		Ray {origin: origin, dir: dir}
	}
	pub fn at(&self, t: f32) -> Vec3{
		self.origin + t * self.dir
	}
}