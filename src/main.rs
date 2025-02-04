use std::cmp::max;
use std::f32::INFINITY;

use indicatif::ProgressBar;

pub mod rtweekend;
pub mod vec3;
pub mod ray;
pub mod hittable;
pub mod hittable_list;
pub mod sphere;
pub mod interval;
pub use crate::hittable::*;
pub use crate::hittable_list::*;
pub use crate::sphere::*;



fn ray_color(r: &Ray, world: &HittableList) -> Color {
	let mut rec = HitRecord::empty();
	if world.hit(r, Interval::new(0.0, INFINITY), &mut rec) {
		return 0.5 * (rec.normal + Color::new(1.0, 1.0, 1.0));
	}

	let a = 0.5 * (r.dir.normalized().y + 1.0);
	(1.0-a)*Color{x: 1.0, y: 1.0, z: 1.0} + a*Color{x: 0.5, y: 0.7, z: 1.0}
}

fn main() {
	// Image
	let aspect_ratio: f32 = 16.0 / 9.0;
	let image_width: i32 = 400;
	let image_height: i32 = max((image_width as f32 / aspect_ratio) as i32, 1);

	// World
	let mut world = HittableList::new();
	world.add(Box::new(Sphere{center: Point3::new(0.0, 0.0, -1.0), radius: 0.5}));
	world.add(Box::new(Sphere{center: Point3::new(0.0, -100.5, -1.0), radius: 100.0}));

	// Camera
	let focal_lenght = 1.0;
	let viewport_height = 2.0;
	let viewport_width = viewport_height * (image_width as f32 / image_height as f32);
	let camera_center = Point3{x: 0.0, y: 0.0, z: 0.0};

	// Calculate the vectors across the horizontal and down the vertical viewport edges.
	let viewport_u = Vec3{x: viewport_width, y: 0.0, z: 0.0};
	let viewport_v = Vec3{x: 0.0, y: -viewport_height, z: 0.0};

	// Calculate the horizontal and vertical delta vectors from pixel to pixel.
	let pixel_delta_u = viewport_u / image_width as f32;
	let pixel_delta_v = viewport_v / image_height as f32;

	// Calculate the location of the upper left pixel.
	let viewport_upper_left = camera_center - Vec3{x: 0.0, y: 0.0, z: focal_lenght} - viewport_u/2.0 - viewport_v/2.0;
	let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

	// Render
	let bar = ProgressBar::new(image_height as u64);

	println!("P3\n{} {}\n255", image_width, image_height);

	for y in 0..image_height {
		for x in 0..image_width{
			let pixel_center = pixel00_loc + (x as f32 * pixel_delta_u) + (y as f32 * pixel_delta_v);
			let ray_dir = pixel_center - camera_center;
			let r = Ray{origin: camera_center, dir: ray_dir};

			let pixel_color = ray_color(&r, &world);
			write_color(&pixel_color);

		}
		bar.inc(1);
	}
}