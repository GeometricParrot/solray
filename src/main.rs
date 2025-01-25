use indicatif::ProgressBar;

mod vec3;
use crate::vec3::*;

fn main() {
	let image_width = 256;
	let image_height = 256;
	let bar = ProgressBar::new(image_height);

	println!("P3\n{} {}\n255", image_width, image_height);

	for y in 0..image_height {
		for x in 0..image_width{

			let outCol = Color{
				x: x as f32 / (image_width - 1) as f32,
				y: y as f32 / (image_height - 1) as f32,
				z: 0.0,
			};
			write_color(&outCol);

		}
		bar.inc(1);
	}
}