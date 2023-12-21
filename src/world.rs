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

	pub fn print_world(&self) {
		for i in 0..self.num_rows {
			for j in 0..self.num_cols {
				let cell = self.get_cell(i, j).unwrap();
				if (cell.get_liveness()) {
					print!("{} ", "1");
				} else {
					print!("{} " , "0");
				}
			}
			println!("{}", "");
		}
	}

	pub fn get_cell(&self, row: usize, col:usize) -> Option<Cell> {
		if (row < self.num_rows && col < self.num_cols) {
			Some(self.cells[row * self.num_cols + col])
		} else {
			None
		}
	}

	pub fn set_cell(&mut self, row: usize, col: usize, cell: Cell) {
		self.cells[row * self.num_cols + col] = cell;
	}

	pub fn mutate(&mut self) {
		for i in 0..self.num_rows {
			for j in 0..self.num_cols {
				let mut cur_cell = self.get_cell(i, j).unwrap();
				self.mutate_helper(&mut cur_cell, i, j);
			}
		}

		for i in 0..self.num_rows {
			for j in 0..self.num_cols {
				let mut cell = self.get_cell(i, j).unwrap();
				if cell.get_liveness() && [2, 3].iter().any(|&i| i == cell.get_alive_neighbors()) {
					cell.set_liveness(true); // keep alive
				} else if (!cell.get_liveness() && [3].iter().any(|&i| i == cell.get_alive_neighbors())) {
					cell.set_liveness(true); // born
				} else {
					cell.set_liveness(false); // dies otherwise
				}

				// reset alive_neighbors
				cell.set_alive_neighbors(0);
				self.set_cell(i, j, cell);
			}
		}
		
	}

	pub fn mutate_helper(&mut self, cur_cell: &mut Cell, i: usize, j: usize) {
		// helper for setting alive_Neighbors
		// iterate through the offsets of neighbors
		for x_offset in [-1, 0, 1].iter().cloned() {
			for y_offset in [-1, 0, 1].iter().cloned() {
				if (x_offset == 0 && y_offset == 0) {
					continue;
				}
				let r = (i as isize) + x_offset;
				let c = (j as isize) + y_offset;
				if (r < 0 || r >= (self.num_rows as isize) || c < 0 || c >= (self.num_cols as isize)) {
					continue;
				}
				// we already checked bounaries above
				let mut cell = self.get_cell(r as usize, c as usize).unwrap();
				if (cell.get_liveness()) {
					cur_cell.set_alive_neighbors(cur_cell.get_alive_neighbors() + 1);
				}
				self.set_cell(i, j, *cur_cell);
			}
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
		let world_1 = World::new(n_rows, n_cols);
		let world_2 = World::new(n_rows, n_cols);

		// "check" randomness of cell liveness
		assert!(world_1 != world_2);
	}

	#[test]
	fn test_mutation() {
		let n_rows = 3;
		let n_cols = 3;
		let mut world_1 = World::new(n_rows, n_cols);
		println!("Before mutating...");
		world_1.print_world();
		world_1.mutate();
		println!("After mutating...");
		world_1.print_world();
		assert!(1==1);
	}
}
