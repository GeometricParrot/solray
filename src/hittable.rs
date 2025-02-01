pub use crate::ray::*;

#[derive(Debug, Copy, Clone)]
pub struct HitRecord {
	pub point: Point3,
	pub normal: Vec3,
	pub t: f32,
	pub front_face: bool,
}

impl HitRecord {
	pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
		// Sets the hit record normal vector.
		// NOTE: the parameter `outward_normal` is assumed to have unit length.

		self.front_face = Vec3::dot(&r.dir, outward_normal) < 0.0;
		self.normal = if self.front_face {*outward_normal} else {-*outward_normal};
	}
}

pub trait Hittable {
	fn hit(&self, r: &Ray, ray_tmin: f32, ray_tmax: f32, record: &mut HitRecord) -> bool;
}
