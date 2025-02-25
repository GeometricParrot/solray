pub use crate::rtweekend::*;

#[derive(Debug, Copy, Clone)]
pub struct HitRecord {
	pub point: Point3,
	pub normal: Vec3,
	pub mat: Material,
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
		HitRecord {
			point: Point3::new(0.0, 0.0, 0.0),
			normal: Vec3::new(0.0, 0.0, 0.0),
			mat: Material::None,
			t: 0.0,
			front_face: false,
		}
	}
}

pub enum Hittable {
	Shpere {
		center: Point3,
		radius: f32,
		mat: Material,
	},
}

impl Hittable {
	pub fn hit(&self, r: &Ray, ray_t: Interval, record: &mut HitRecord) -> bool {
		match self {
			Hittable::Shpere {center, radius, mat } => {
				let oc: Vec3 = *center - r.origin;
				let a = r.dir.length_squared();
				let h = Vec3::dot(&r.dir, &oc);
				let c = oc.length_squared() - radius * radius;

				let discriminant = h*h - a * c;
				if discriminant < 0.0 {
					return false;
				}
				let sqrtd = discriminant.sqrt();

				// Find the nearest root that lies in the acceptable range.
				let mut root = (h - sqrtd) / a;
				if !ray_t.surrounds(root) {
					root = (h + sqrtd) / a;
					if !ray_t.surrounds(root) {
						return false
					}
				}

				record.t = root;
				record.point = r.at(record.t);
				let outward_normal = (record.point - *center) / *radius;
				record.set_face_normal(r, &outward_normal);
				record.mat = *mat;

				return true
			}
		}
		
	}
}