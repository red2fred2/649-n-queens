use rand::{prelude::ThreadRng, Rng};

const BOARD_SIZE: usize = 4;

fn main() {
	// Set up random number generator
	let mut rng = rand::thread_rng();

	// Create random board to start
	let board = random_board(&mut rng);
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
				println!("col {i} sees a row attack against col {j}");
			}
		}

		// Check if any are in a diagonal from here
		for j in 0..BOARD_SIZE {
			let vertical_gap = row as i8 - board[j] as i8;

			if i != j && i as i8 - j as i8 == vertical_gap {
				attacks += 1;
				println!("col {i} sees desc attack against col {j}");
			}
			if i != j && j as i8 - i as i8 == vertical_gap {
				attacks += 1;
				println!("col {i} sees asc attack col {j}");
			}
		}
	}

	// Divide b two because any queen that can be attacked can also attack the
	// one they're vulnerable to.
	attacks / 2
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
