pub use crate::rtweekend::*;

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
	pub fn empty() -> HitRecord {
		HitRecord{
			point: Point3::new(0.0, 0.0, 0.0),
			normal: Vec3::new(0.0, 0.0, 0.0),
			t: 0.0,
			front_face: false,
		}
	}
}

pub trait Hittable {
	fn hit(&self, r: &Ray, ray_t: Interval, record: &mut HitRecord) -> bool;
}
