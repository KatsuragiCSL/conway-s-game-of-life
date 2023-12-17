#[derive(Copy, Clone)]
pub struct Cell {
	generation: u64,
	liveness: bool,
	alive_neighbors: u64,
}

impl Cell {
	pub fn new(alive: bool) -> Cell {
		Cell {
			generation: 0,
			liveness: alive,
			alive_neighbors: 0,
		}
	}

	pub fn get_alive_neighbors(&self) -> u64 {
		self.alive_neighbors
	}

	pub fn inc_alive_neighbors(&mut self) {
		self.alive_neighbors += 1;
	}

	pub fn get_liveness(&mut self) -> bool {
		self.liveness
	}
	
	pub fn set_liveness(&mut self, alive: bool) {
		self.liveness = alive;
	}

	pub fn get_generation(&self) -> u64 {
		self.generation
	}
	
	pub fn inc_generation(&mut self) {
		self.generation += 1;
	}

}

