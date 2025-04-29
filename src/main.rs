pub mod rtweekend;
pub mod vec3;
pub mod ray;
pub mod hittable;
pub mod hittable_list;
pub mod interval;
pub mod camera;
pub mod material;
use camera::Camera;

pub use crate::hittable::*;
pub use crate::hittable_list::*;

fn main() {
	let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(121);
	// World
	let mut world = HittableList::new();
	// floor
	world.add( Box::new( Hittable::Shpere {
		center: Point3::new(0.0, -1001.0, 0.0),
		radius: 1000.0,
		mat: Material::Lambertian(Color::new(0.9, 0.9, 0.9))})
	);

	for x in -10..10 {
		for z in -10..10 {
			let random = rng.random_range(0.0..1.0);

			let col = Color::new(
				(rng.random_range(0.0..1.0) as f32).powi(2),
				(rng.random_range(0.0..1.0) as f32).powi(2),
				(rng.random_range(0.0..1.0) as f32).powi(2),
			);

			world.add( Box::new( Hittable::Shpere {
				center: Point3::new(x as f32 + rng.random_range(-0.5..0.5), rng.random_range(-1.0..2.0), z as f32 + rng.random_range(-0.5..0.5)),
				radius: 0.25 + (rng.random_range(0.0..1.1) as f32).powi(3) / 2.0,

				mat: if random < 0.5 {
					Material::Lambertian(col)
				} else  if random < 0.9 {
					Material::Metal(col, (rng.random_range(0.0..0.9) as f32).powi(2))
				} else {
					Material::Dielectric(rng.random_range(1.1..1.9))
				}
				
			}));
		}
	}

	let camera = Arc::new(Camera::new(
		16.0 / 9.0,
		2560 / 4,
		60.0,
		1000 / 4,
		Point3::new(3.0, 3.0, 13.0),
		Point3::new(0.0, 0.0, 0.0),
		Vec3::new(0.0, 1.0, 0.0),
		0.25,
		5.0,
	));

	let (tx, rx) = mpsc::channel();

	Camera::render(camera.clone(), Arc::new(world), tx, &mut rng);

	let bar = ProgressBar::new((camera.image_height * camera.image_width) as u64);
	let mut buffer = vec![Color::new(0.0, 0.0, 0.0); (camera.image_height * camera.image_width) as usize];
	for (color, x, y) in rx {
		buffer[(x + (y * camera.image_width)) as usize] = color;
		bar.inc(1);
	}

	for color in buffer {
		write_color(&color);
	}
}