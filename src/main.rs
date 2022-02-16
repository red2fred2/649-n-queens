use std::collections::btree_map::Range;

use rand::{prelude::ThreadRng, Rng};

const BOARD_SIZE: usize = 8;

fn main() {
	// Set up random number generator
	let mut rng = rand::thread_rng();

	// Create blank board
	let board: Vec<usize> = Vec::new();
}

/// Returns a board with random data
fn random_board(rng: &mut ThreadRng) -> Vec<usize> {
	let mut board = Vec::new();

	for _ in 0..BOARD_SIZE {
		// Generate a random row number
		let row = rng.gen_range(0..BOARD_SIZE);

		// Add it to the board
		board.push(row);
	}

	board
}
