pub use crate::rtweekend::*;
use crate::hittable::*;

#[derive(Debug, Copy, Clone)]
pub enum Material {
	Lambertian(Color),
	Metal(Color),
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
			Material::Metal(albedo) => {
				let reflected = Vec3::reflect(&r_in.dir, &record.normal);
				*scattered = Ray::new(record.point, reflected);
				*attenuation = *albedo;
				true
			},
			_ => {
				panic!("in 'Material::scatter' unhangled material");
				false
			},
		}
	}
}