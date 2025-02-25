pub use crate::rtweekend::*;
use crate::hittable::*;

#[derive(Debug, Copy, Clone)]
pub enum Material {
	Lambertian(Color),
	Metal(Color, f32),
	None,
}

impl Material {
	pub fn scatter(&self, r_in: &Ray, record: HitRecord, attenuation: &mut Color, scattered: &mut Ray, rng: &mut ChaCha8Rng) -> bool {
		match self {
			Material::Lambertian(albedo) => {
				let random_scatter_direction = record.normal + Vec3::random_unit_vector(rng);
				// Catch degenerate scatter direction
				let scatter_direction = if random_scatter_direction.near_zero() {record.normal} else {random_scatter_direction};
				*scattered = Ray::new(record.point, scatter_direction);
				*attenuation = *albedo;
				true
			},
			Material::Metal(albedo, fuzz) => {
				let reflected = Vec3::reflect(&r_in.dir, &record.normal).normalized() + (*fuzz * Vec3::random_unit_vector(rng));
				*scattered = Ray::new(record.point, reflected);
				*attenuation = *albedo;
				return Vec3::dot(&scattered.dir, &record.normal) > 0.0;
			},
			_ => {
				panic!("in 'Material::scatter' unhangled material");
				false
			},
		}
	}
}