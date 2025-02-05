use indicatif::ProgressBar;

pub use crate::rtweekend::*;
pub use crate::hittable_list::*;

pub struct Camera {
	pub aspect_ratio: f32,
	pub image_width: i32,
	image_height: i32,
	center: Point3,
	pixel00_loc: Point3,
	pixel_delta_u: Vec3,
	pixel_delta_v: Vec3,

}

impl Camera {
	pub fn ray_color(&self, r: &Ray, world: &HittableList) -> Color {
		let mut rec = HitRecord::empty();
		if world.hit(r, Interval::new(0.0, INFINITY), &mut rec) {
			return 0.5 * (rec.normal + Color::new(1.0, 1.0, 1.0));
		}

		let a = 0.5 * (r.dir.normalized().y + 1.0);
		(1.0-a)*Color::new(1.0, 1.0, 1.0) + a*Color::new(0.5, 0.7, 1.0)
	}

	pub fn new(aspect_ratio: f32, image_width: i32) -> Camera {
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

		Camera{
			aspect_ratio: aspect_ratio,
			image_width: image_width,
			image_height: image_height,
			center: center,
			pixel00_loc: pixel00_loc,
			pixel_delta_u: pixel_delta_u,
			pixel_delta_v: pixel_delta_v,
		}
	}

	pub fn render(&mut self, world: &HittableList) {
		// initalize call is forgone
		println!("P3\n{} {}\n255", self.image_width, self.image_height);

		let bar = ProgressBar::new(self.image_height as u64);

		for y in 0..self.image_height {
			for x in 0..self.image_width{
				let pixel_center = self.pixel00_loc + (x as f32 * self.pixel_delta_u) + (y as f32 * self.pixel_delta_v);
				let ray_dir = pixel_center - self.center;
				let r = Ray{origin: self.center, dir: ray_dir};

				let pixel_color = self.ray_color(&r, &world);
				write_color(&pixel_color);

			}
			bar.inc(1);
		}
	}
}