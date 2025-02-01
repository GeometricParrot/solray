pub use crate::vec3::*;

#[derive(Debug, Copy, Clone)]
pub struct Ray {
	pub origin: Point3,
	pub dir: Vec3,
}

impl Ray {
	pub fn at(&self, t: f32) -> Vec3{
		self.origin + t * self.dir
	}
}