pub struct Interval {
	pub min: f32,
	pub max: f32,
}

impl Interval {
	pub fn new(min: f32, max: f32) -> Interval {
		Interval{min: min, max: max}
	}
	pub fn size(&self) -> f32 {
		self.max - self.min
	}
	pub fn contains(&self, n: f32) -> bool {
		self.min <= n && self.max >= n
	}
	pub fn surrounds(&self, n: f32) -> bool {
		self.min < n && self.max > n
	}
}