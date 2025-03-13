pub use crate::rtweekend::*;
pub use crate::hittable_list::*;

pub struct Camera {
	pub aspect_ratio: f32,
	pub image_width: i32,
	pub samples_per_pixel: i32,
	image_height: i32,
	center: Point3,
	pixel00_loc: Point3,
	pixel_delta_u: Vec3,
	pixel_delta_v: Vec3,
	pixel_samples_scale: f32,
	max_depth: i32,
}

impl Camera {
	#[deprecated]
	pub fn legacy_ray_color(&self, r: &Ray, world: &HittableList, depth: i32, rng: &mut ChaCha8Rng) -> Color {
		if depth <= 0 {
			return Color::new(0.0, 0.0, 0.0);
		}
		let mut rec = HitRecord::empty();
		if world.hit(r, Interval::new(0.001, INFINITY), &mut rec) {
			let mut scattered = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0));
			let mut attenuation = Color::new(0.0, 0.0, 0.0);

			if rec.mat.scatter(r, rec, &mut attenuation, &mut scattered, rng) {
				return attenuation * self.legacy_ray_color(&scattered, world, depth-1, rng);
			}
			else {
				return Color::new(0.0, 0.0, 0.0);
			}
		}

		let a = 0.5 * (r.dir.normalized().y + 1.0);
		return (1.0-a)*Color::new(1.0, 1.0, 1.0) + a*Color::new(0.5, 0.7, 1.0);
	}

	pub fn ray_color(mut r:  Ray, world: &HittableList, depth: i32, rng: &mut ChaCha8Rng) -> Color {
		// this is multiplyed by attinuation each iteration
		let mut output_color = Color::white();
		let mut temp_attenuation = Color::black();
		let mut temp_scattered = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0));
		for _ in 0..depth {
			// if the ray hit something
			let mut rec = HitRecord::empty();
			if world.hit(&r, Interval::new(0.001, INFINITY), &mut rec) {
				// if the ray is reflected
				if rec.mat.scatter(&r, rec, &mut temp_attenuation, &mut temp_scattered, rng) {
					output_color *= temp_attenuation;
					r = temp_scattered;
					continue
				}
				// else ray was absorbed, did not reach light source
				else {
					return Color::black();
				}
			}
			// ray hit nothing, therefore sky light
			else {
				let a = 0.5 * (r.dir.normalized().y + 1.0);
				output_color *= (1.0-a)*Color::white() + a*Color::new(0.5, 0.7, 1.0);
				return output_color;
			}
		}
		return Color::black();
	}	

	pub fn new(aspect_ratio: f32, image_width: i32, samples_per_pixel: i32) -> Camera {
		//let aspect_ratio = 16.0 / 9.0;
		//let image_width = 400;
		let image_height = max((image_width as f32 / aspect_ratio) as i32, 1);
		let center = Point3::new(0.0, 0.0, 0.0);


		// Determine viewport dimensions.
		let focal_lenght = 1.0;
		let viewport_height = 2.0;
		let viewport_width = viewport_height * (image_width as f32 / image_height as f32);

		// Calculate the vectors across the horizontal and down the vertical viewport edges.
		let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
		let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

		// Calculate the horizontal and vertical delta vectors from pixel to pixel.
		let pixel_delta_u = viewport_u / image_width as f32;
		let pixel_delta_v = viewport_v / image_height as f32;

		// Calculate the location of the upper left pixel.
		let viewport_upper_left = center - Vec3::new(0.0, 0.0, focal_lenght) - viewport_u/2.0 - viewport_v/2.0;
		let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

		let pixel_samples_scale = 1.0 / samples_per_pixel as f32;

		Camera{
			aspect_ratio: aspect_ratio,
			image_width: image_width,
			samples_per_pixel: samples_per_pixel,
			image_height: image_height,
			center: center,
			pixel00_loc: pixel00_loc,
			pixel_delta_u: pixel_delta_u,
			pixel_delta_v: pixel_delta_v,
			pixel_samples_scale: pixel_samples_scale,
			max_depth: 10,
		}
	}

	fn sample_square(&self, rng: &mut ChaCha8Rng) -> Vec3 {
		Vec3::new(rng.random_range(-0.5..0.5), rng.random_range(-0.5..0.5), 0.0)
	}

	fn get_ray(&self, x: i32, y: i32, rng: &mut ChaCha8Rng) -> Ray {
		// Construct a camera ray originating from the origin and directed at randomly sampled
		// point around the pixel location x, y.
		
		let offset = self.sample_square(rng);
		let pixel_sample = self.pixel00_loc
			+ (x as f32 + offset.x) * self.pixel_delta_u
			+ (y as f32 + offset.y) * self.pixel_delta_v;
		
		Ray{origin: self.center, dir: pixel_sample - self.center}
	}

	pub fn render(&self, world: &HittableList, rng: &mut ChaCha8Rng) {
		// initalize call is forgone
		println!("P3\n{} {}\n255", self.image_width, self.image_height);

		let bar = ProgressBar::new(self.image_height as u64);

		for y in 0..self.image_height {
			for x in 0..self.image_width{
				let mut pixel_color = Color::new(0.0, 0.0, 0.0);
				for _sample in 0..self.samples_per_pixel {
					let r = self.get_ray(x, y, rng);
					pixel_color += Camera::ray_color(r, &world, self.max_depth, rng);
				}
				//let pixel_center = self.pixel00_loc + (x as f32 * self.pixel_delta_u) + (y as f32 * self.pixel_delta_v);
				//let ray_dir = pixel_center - self.center;
				//let r = Ray{origin: self.center, dir: ray_dir};

				write_color(&(pixel_color * self.pixel_samples_scale));

			}
			bar.inc(1);
		}
	}
}