use rand::{prelude::ThreadRng, Rng};

const BOARD_SIZE: usize = 8;
const T0: f32 = 30.;
const A: f32 = 0.998;

fn main() {
	// Set up random number generator
	let mut rng = rand::thread_rng();

	// let (board, fitness_checks) = random_restart_hill_climb(&queens_attacking, &mut rng);
	let (board, fitness_checks) = simulated_annealing(&queens_attacking, &mut rng);

	// Print resulting board
	println!("-------------------------------------------------------------");
	print_board(&board);
	print_board_as_stupid_string(&board);
	let attacks = queens_attacking(&board);
	println!("attacks: {attacks}");
	println!("Fitness checks: {fitness_checks}");
}

fn simulated_annealing(
	fitness_fn: &dyn Fn(&Vec<usize>) -> u8,
	rng: &mut ThreadRng
) -> (Vec<usize>, u64) {
	let mut board = random_board(rng);
	let mut fitness_checks = 1;
	let mut current_fitness = fitness_fn(&board);
	let mut temperature = T0;

	loop {
		temperature = temperature * A;

		// If T = 0, return
		if temperature < f32::MIN_POSITIVE {
			return (board, fitness_checks)
		}

		// Check a random board
		let next_board = random_successor(&board, rng);
		let next_fitness = fitness_fn(&next_board);
		let fitness_diff = current_fitness as i16 - next_fitness as i16;
		fitness_checks += 1;

		// Check if this board should replace the last
		if fitness_diff > 0 {
			board = next_board;
			current_fitness = next_fitness;
		} else {
			// Calculate second chance
			let rand = rng.gen_range(0.0..1.0);
			let exponent = fitness_diff as f32 / temperature;
			let probability = std::f32::consts::E.powf(exponent);

			// If the second chance succeeds, set the board anyway
			if probability > rand {
				board = next_board;
				current_fitness = next_fitness;
			}
		}
	}
}

// Makes a random move on the board
fn random_successor(board: &Vec<usize>, rng: &mut ThreadRng) -> Vec<usize> {
	// Pick a random column to set to a random row
	let col = rng.gen_range(0..BOARD_SIZE);
	let row = rng.gen_range(0..BOARD_SIZE);

	// Make that move
	let mut new_board = board.clone();
	new_board[col] = row;
	new_board
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

// Prints the solution as a hard to understand string
fn print_board_as_stupid_string(board: &Vec<usize>) {
	print!("Solution: ");
	for row in board {
		print!("{row}");
	}
	println!();
}