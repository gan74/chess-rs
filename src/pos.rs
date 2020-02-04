
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pos {
	i: u8
}

impl Pos {
	pub fn new(x: usize, y: usize) -> Pos {
		debug_assert!(x < 8);
		debug_assert!(y < 8);
		Pos {
			i: (x + y * 8) as u8
		}
	}

	pub fn from_index(index: usize) -> Pos {
		debug_assert!(index < 64);
		Pos {
			i: index as u8
		}
	}

	pub fn index(&self) -> usize {
		self.i as usize
	}

	pub fn col(&self) -> usize {
		(self.i % 8) as usize
	}

	pub fn row(&self) -> usize {
		(self.i / 8) as usize
	}
}