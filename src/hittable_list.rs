pub use crate::hittable::*;

pub struct HittableList
{
	pub objects: Vec<Box<Hittable>>,
}

impl HittableList
{
	pub fn add(&mut self, object: Box<Hittable>) {
		self.objects.push(object);
	}
	pub fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
		let mut temp_rec: HitRecord = HitRecord::empty();
		let mut hit_anything = false;
		let mut closest_so_far = ray_t.max;

		for object in &self.objects {
			if object.hit(r, Interval::new(ray_t.min, closest_so_far), &mut temp_rec) {
				hit_anything = true;
				closest_so_far = temp_rec.t;
				*rec = temp_rec;
			}
		}

		return hit_anything;
	}
	pub fn new() -> HittableList {
		HittableList{objects: vec![]}
	}
}