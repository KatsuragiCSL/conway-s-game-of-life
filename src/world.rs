use nannou::rand::{thread_rng, Rng};
use super::cell::Cell;

pub struct World {
	num_rows: usize,
	num_cols: usize,
	cells: Vec<Cell>,
}

impl World {
	pub fn new(num_rows: usize, num_cols: usize) -> World {
		let mut rng = nannou::rand::thread_rng();
		World {
			num_rows: num_rows,
			num_cols: num_cols,
			//cells: vec![Cell::new(rng.gen_bool(0.5)); num_rows * num_cols],
			cells: (0..(num_rows * num_cols)).map(|_| Cell::new(rng.gen_bool(0.5))).collect(),
		}
	}

	pub fn get_cell(&self, row: usize, col:usize) -> Option<Cell> {
		if (row < self.num_rows && col < self.num_cols) {
			Some(self.cells[row * self.num_cols + col])
		} else {
			None
		}
	}
}

#[cfg(test)]
mod test {
	use super::*;

	impl PartialEq for World {
    	fn eq(&self, other: &Self) -> bool {
			if self.num_rows != other.num_rows || self.num_cols == other.num_cols {
				return false
			}
			for i in 0..self.num_rows {
				for j in 0..self.num_cols {
					if (self.get_cell(i, j).unwrap().get_liveness() != other.get_cell(i, j).unwrap().get_liveness()) {
						return false
					}
				}
			}
			return true
    	}
	}
	impl Eq for World {}

	#[test]
	fn test_randomness_of_worlds() {
		let n_rows = 4;
		let n_cols = 3;
		let mut world_1 = World::new(n_rows, n_cols);
		let mut world_2 = World::new(n_rows, n_cols);

		// "check" randomness of cell liveness
		assert!(world_1 != world_2);
	}
}
