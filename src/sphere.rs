pub use crate::hittable::*;

pub struct Sphere {
	center: Point3,
	radius: f32,
}

impl Sphere {
	pub fn new(center: Point3, radius: f32) -> Sphere{
		Sphere {
			center: center,
			radius: radius,
		}
	}
}

impl Hittable for Sphere {
	fn hit(&self, r: &Ray, ray_tmin: f32, ray_tmax: f32, record: &mut HitRecord) -> bool {
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
		if root <= ray_tmin || root >= ray_tmax {
			root = (h + sqrtd) / a;
			if root <= ray_tmin || root >= ray_tmax {
				return false
			}
		}

		record.t = root;
		record.point = r.at(record.t);
		let outward_normal = (record.point - self.center) / self.radius;
		record.set_face_normal(r, &outward_normal);

		return true
	}
}