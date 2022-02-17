use std::ptr;

use rand::{prelude::ThreadRng, Rng};

const BOARD_SIZE: usize = 8;

fn main() {
	// Set up random number generator
	let mut rng = rand::thread_rng();

	// Create random board to start
	let mut board = random_board(&mut rng);
	let attacks = queens_attacking(&board);

	println!("attacks: {attacks}");
	print_board(&board);
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

	// Go column by column
	for i in 0..BOARD_SIZE {
		let row = board[i];

		// Check if any queens are in the same row
		for j in 0..BOARD_SIZE {
			// If a different column has the same row value
			if j != i && board[j] == row {
				attacks += 1;
			}
		}

		// Check if any are in a diagonal from here
		for j in 0..BOARD_SIZE {
			let vertical_gap = row as isize - board[j] as isize;

			if i as isize - j as isize == vertical_gap {
				attacks += 1;
			}
			if j as isize- i as isize == vertical_gap {
				attacks += 1;
			}
		}
	}

	attacks
}

/// Prints out a current board
///
/// The code is gross, but not important, so I'm not going to make an effort
fn print_board(board: &Vec<usize>) {
	let mut print_board: Vec<Vec<bool>> = Vec::new();

	// Set up easier board to print out
	for r in 0..BOARD_SIZE {
		let mut row = Vec::new();
		for c in 0..BOARD_SIZE {
			if board[c] == r {
				row.push(true);
			} else {
				row.push(false);
			}
		}
		print_board.push(row);
	}

	// Print board top
	for _ in 0.. BOARD_SIZE {
		print!("+---");
	}
	println!("+");

	// Print the rest of the board
	for row in print_board {
		for col in row {
			if col {
				print!("| X ");
			} else {
				print!("|   ");
			}
		}
		println!("|");
		for _ in 0.. BOARD_SIZE {
			print!("+---");
		}
		println!("+");
	}
}
