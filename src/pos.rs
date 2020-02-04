pub struct Pos {
	x: u8,
	y: u8
}

impl Pos {
	pub fn new(x: usize, y: usize) -> Pos {
		debug_assert!(x < 8);
		debug_assert!(y < 8);
		Pos {
			x: x as u8,
			y: y as u8
		}
	}

	pub fn index(&self) -> usize {
		(self.x + self.y * 8) as usize
	}
}