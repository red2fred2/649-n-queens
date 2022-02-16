use std::collections::btree_map::Range;

use rand::{prelude::ThreadRng, Rng};

const BOARD_SIZE: usize = 8;

fn main() {
	// Set up random number generator
	let mut rng = rand::thread_rng();

	// Create random board to start
	let mut board = random_board(&mut rng);
	let attacks = queens_attacking(&board);
	println!("attacks: {attacks}");

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

/// Finds the number of queens attacking
fn queens_attacking(board: &Vec<usize>) -> u8 {
	let mut attacks = 0;
	let board_len = board.len();

	// Go column by column
	for i in 0..board_len {
		let row = board[i];

		// Check if any queens are in the same row
		for j in 0..board_len {
			// If a different column has the same row value
			if j != i && board[j] == row {
				attacks += 1;
			}
		}

		// Check if any are in a diagonal from here
		for j in 0..board_len {
			let vertical_gap = row as i8 - board[j] as i8;

			if i as i8 - j as i8 == vertical_gap {
				attacks += 1;
			}
			if j as i8 - i as i8 == vertical_gap {
				attacks += 1;
			}
		}
	}

	attacks
}
