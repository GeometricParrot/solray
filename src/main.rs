pub mod rtweekend;
pub mod vec3;
pub mod ray;
pub mod hittable;
pub mod hittable_list;
pub mod sphere;
pub mod interval;
pub mod camera;
pub mod material;
use camera::Camera;

pub use crate::hittable::*;
pub use crate::hittable_list::*;
pub use crate::sphere::*;

fn main() {
	let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(123);
	// World
	let mut world = HittableList::new();
	world.add( Box::new( Sphere{
		center: Point3::new(0.0, -100.5, -1.0),
		radius: 100.0,
		mat: Material::Lambertian(Color::new(0.8, 0.8, 0.0))}));
	world.add( Box::new( Sphere{
		center: Point3::new(0.0, 0.0, -1.2),
		radius: 0.5,
		mat: Material::Lambertian(Color::new(0.1, 0.2, 0.5))}));
	world.add( Box::new( Sphere{
		center: Point3::new(-1.0, 0.0, -1.0),
		radius: 0.5,
		mat: Material::Metal(Color::new(0.8, 0.8, 0.8), 0.3)}));
	world.add( Box::new( Sphere{
		center: Point3::new(1.0, 0.0, -1.0),
		radius: 0.5,
		mat: Material::Metal(Color::new(0.8, 0.6, 0.2), 1.0)}));

	let camera = Camera::new(16.0 / 9.0, 400, 300);
	camera.render(&world, &mut rng);
}