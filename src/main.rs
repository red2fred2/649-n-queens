use rand::{prelude::ThreadRng, Rng};

const BOARD_SIZE: usize = 5;

fn main() {
	// Set up random number generator
	let mut rng = rand::thread_rng();

	// Create random board to start
	let mut board = random_board(&mut rng);

	// Print board before algorithm
	print_board(&board);
	let attacks = queens_attacking(&board);
	println!("Attacks: {attacks}");

	// Do rrhc
	random_restart_hill_climb(&mut board, &queens_attacking, &mut rng);

	// Print resulting board
	println!("-------------------------------------------------------------");
	print_board(&board);
	let attacks = queens_attacking(&board);
	println!("Attacks: {attacks}");
}

// Do a random restart hill climbing algorithm until the fitness function returns 0
fn random_restart_hill_climb(
	board: &mut Vec<usize>,
	fitness_fn: &dyn Fn(&Vec<usize>) -> u8,
	rng: &mut ThreadRng
) {
	// Random hill climb
	hill_climb(board, fitness_fn, rng);
}

// Climbs up until no real change is happening
fn hill_climb(
	board: &mut Vec<usize>,
	fitness_fn: &dyn Fn(&Vec<usize>) -> u8,
	rng: &mut ThreadRng
) -> u8 {
	let mut scores = Vec::new();

	loop {
		// Pick a random column and step in it
		let col = rng.gen_range(0..BOARD_SIZE);
		let fitness = hill_step(board, fitness_fn, col);

		// Stop working if the board is perfect
		if fitness == 0 {
			return fitness
		}

		// Give up if there hasn't been much improvement recently
		scores.push(fitness);

		let last_scores = scores.iter().rev().take(BOARD_SIZE*2);
		let sum_of_last_scores: u8 = last_scores.sum();
		let avg_of_last_scores = sum_of_last_scores / BOARD_SIZE as u8 / 2;

		if scores.len() > BOARD_SIZE * 2 && avg_of_last_scores == fitness {
			return fitness
		}
	}
}

/// Does 1 single step on one column in a hill climb
fn hill_step(
	board: &mut Vec<usize>,
	fitness_fn: &dyn Fn(&Vec<usize>) -> u8,
	col: usize
) -> u8 {
	let mut best_row = board[col];
	let mut best_fitness = u8::MAX;

	// Go through possible moves
	for row in 0..BOARD_SIZE {
		// Change then test
		board[col] = row;
		let fitness = fitness_fn(board);

		// Change if better
		if fitness < best_fitness {
			best_fitness = fitness;
			best_row = row;
		}
	}
	// Set best fitness
	board[col] = best_row;
	best_fitness
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
				// println!("col {i} sees a row attack against col {j}");
			}
		}

		// Check if any are in a diagonal from here
		for j in 0..BOARD_SIZE {
			let vertical_gap = row as i8 - board[j] as i8;

			if i != j && i as i8 - j as i8 == vertical_gap {
				attacks += 1;
				// println!("col {i} sees desc attack against col {j}");
			}
			if i != j && j as i8 - i as i8 == vertical_gap {
				attacks += 1;
				// println!("col {i} sees asc attack col {j}");
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
