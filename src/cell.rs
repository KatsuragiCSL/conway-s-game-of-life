#[derive(Copy, Clone)]
pub struct Cell {
	liveness: bool,
	alive_neighbors: u64,
}

impl Cell {
	pub fn new(alive: bool) -> Cell {
		Cell {
			liveness: alive,
			alive_neighbors: 0,
		}
	}

	pub fn get_alive_neighbors(&self) -> u64 {
		self.alive_neighbors
	}

	pub fn set_alive_neighbors(&mut self, an: u64) {
		self.alive_neighbors = an;
	}

	pub fn get_liveness(&self) -> bool {
		self.liveness
	}
	
	pub fn set_liveness(&mut self, alive: bool) {
		self.liveness = alive;
	}

}

