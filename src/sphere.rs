pub use crate::rtweekend::*;
pub use crate::hittable::*;
pub use crate::material::*;

pub struct Sphere {
	pub center: Point3,
	pub radius: f32,
	pub mat: Material,
}

impl Sphere {
	pub fn new(center: Point3, radius: f32, material: Material) -> Sphere {
		Sphere {
			center: center,
			radius: radius,
			mat: material,
		}
	}
}

impl Hittable for Sphere {
	fn hit(&self, r: &Ray, ray_t: Interval, record: &mut HitRecord) -> bool {
		let oc: Vec3 = self.center - r.origin;
		let a = r.dir.length_squared();
		let h = Vec3::dot(&r.dir, &oc);
		let c = oc.length_squared() - self.radius * self.radius;

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
		let outward_normal = (record.point - self.center) / self.radius;
		record.set_face_normal(r, &outward_normal);
		record.mat = self.mat;

		return true
	}
}