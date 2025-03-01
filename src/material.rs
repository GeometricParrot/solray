pub use crate::rtweekend::*;
use crate::hittable::*;

#[derive(Debug, Copy, Clone)]
pub enum Material {
	Lambertian(Color),
	Metal(Color, f32),
	Dielectric(f32),
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
			// Refractive index in vacuum or air, or the ratio of the material's refractive index over
			// the refractive index of the enclosing media
			Material::Dielectric(refraction_index) => {
				*attenuation = Color::new(1.0, 1.0, 1.0);
				let ri = if record.front_face {1.0 / refraction_index} else {*refraction_index};

				let unit_direction = r_in.dir.normalized();
				let cos_theta = f32::min(Vec3::dot(&-unit_direction, &record.normal), 1.0);
				let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

				let cannot_refract = ri * sin_theta > 1.0;
				let direction = 
					if cannot_refract || Material::reflectance(cos_theta, ri) > rng.random_range(0.0..1.0)
						{unit_direction.reflect(&record.normal)}
					else
						{unit_direction.refract(&record.normal, ri)};

				*scattered = Ray::new(record.point, direction);
				return true;
			},
		}
	}
	
	pub fn reflectance(cosine: f32, refraction_index: f32) -> f32{
		// Use Schlick's approximation for reflectance.
		let r0 = ((1.0 - refraction_index) / (1.0 + refraction_index)).powi(2);
		return r0 + (1.0 - r0) * (1.0 - cosine).powi(5)

	}
}