use indicatif::ProgressBar;

fn main() {
	let image_width = 256;
	let image_height = 256;
	let bar = ProgressBar::new(image_height);

	println!("P3\n{} {}\n255", image_width, image_height);

	for y in 0..image_height {
		for x in 0..image_width{
			let r = x as f32 / (image_width - 1) as f32;
			let g = (y as f32) / (image_height - 1) as f32;
			let b = 0.0 as f32;

			let r = (r * 255.999) as i32;
			let g = (g * 255.999) as i32;
			let b = (b * 255.999) as i32;

			println!("{} {} {}", r, g, g);
		}
		bar.inc(1);
	}
}