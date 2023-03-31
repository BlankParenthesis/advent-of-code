use std::collections::HashMap;

use crate::Part;

enum Rock {
	Beam,
	Cross,
	L,
	Pillar,
	Box,
}

impl Rock {
	fn starting_bitmask(&self) -> [u8; 4] {
		match self {
			Rock::Beam => [0b00011110, 0, 0, 0],
			Rock::Cross => [0b00001000, 0b00011100, 0b00001000, 0],
			Rock::L => [0b00011100, 0b00000100, 0b00000100, 0],
			Rock::Pillar => [0b00010000; 4],
			Rock::Box => [0b00011000, 0b00011000, 0, 0],
		}
	}

	fn from_index(index: usize) -> Self {
		match index % 5 {
			0 => Rock::Beam,
			1 => Rock::Cross,
			2 => Rock::L,
			3 => Rock::Pillar,
			4 => Rock::Box,
			_ => panic!("never"),
		}
	}
}

fn test_collision(grid: &[u8], rock: &[u8; 4], height: usize) -> bool {
	for i in 0..4 {
		if rock[i] > 0 {
			if let Some(row) = grid.get(height + i) {
				if row & rock[i] > 0 {
					return true;
				}
			}
		}
	}
	false
}

enum Push {
	Left,
	Right,
}

impl Push {
	fn shift(&self, rock: &[u8; 4]) -> [u8; 4] {
		match self {
			Push::Left => {
				[
					rock[0] << 1,
					rock[1] << 1,
					rock[2] << 1,
					rock[3] << 1,
				]
			},
			Push::Right => {
				[
					rock[0] >> 1,
					rock[1] >> 1,
					rock[2] >> 1,
					rock[3] >> 1,
				]
			},
		}
	}

	fn push(&self, grid:&[u8], rock: &mut [u8; 4], height: usize) {
		const left_check: u8 = 0b01000000;
		const right_check: u8 = 0b00000001;
		let rock_collision = rock[0] | rock[1] | rock[2] | rock[3];

		let collides_walls = match self {
			Push::Left => left_check & rock_collision,
			Push::Right => right_check & rock_collision,
		} > 0;

		if !collides_walls {
			let new_rock = self.shift(rock);
			if !test_collision(grid, &new_rock, height) {
				*rock = new_rock
			}
		}
	}
}


pub(crate) fn solve(data: &[u8], part: Part) {
	let jet_pattern = data.iter().map(|b| match b {
		b'<' => Push::Left,
		b'>' => Push::Right,
		_ => panic!(),
	})
	.collect::<Vec<Push>>();

	//let mut pattern_inf = std::iter::repeat(jet_pattern.iter()).flatten();
	let mut pattern_index = 0;

	// let each row be a byte with hot bits being rock and cold bits being empty
	let mut grid: Vec<u8> = vec![];

	let total_rock_count = match part {
		Part::A => 2022,
		Part::B => 1_000_000_000_000,
	};

	#[derive(Debug, Clone)]
	struct Snapshot {
		height: usize,
		rock_number: usize,
		snapshot: u64,
	}

	let mut rock_memories = vec![HashMap::<usize, Snapshot>::new(); 5];

	let mut extra_height_prediction = None;
	let mut rock_number = 0;
	while rock_number < total_rock_count {
		let mut height = grid.len() + 3;
		let mut rock = Rock::from_index(rock_number).starting_bitmask();

		let rock_index = rock_number % 5;

		let grid_height = grid.len();

		if extra_height_prediction.is_none() && grid_height > 90000 {
			let snapshot = grid[grid_height - 1] as u64
			| (grid[grid_height - 2] as u64) << 8
			| (grid[grid_height - 3] as u64) << 16
			| (grid[grid_height - 4] as u64) << 24
			| (grid[grid_height - 5] as u64) << 32
			| (grid[grid_height - 6] as u64) << 40
			| (grid[grid_height - 7] as u64) << 48
			| (grid[grid_height - 8] as u64) << 56;

			if rock_memories[rock_index].contains_key(&pattern_index)
					&& rock_memories[rock_index].get(&pattern_index).unwrap().snapshot == snapshot {
				
				let memory = rock_memories[rock_index].get(&pattern_index).unwrap();

				let period = rock_number - memory.rock_number;
				let period_height = grid_height - memory.height;
				let remaining_rocks = total_rock_count - rock_number;
				let skipped_cycles = remaining_rocks / period;

				let predicted_height = skipped_cycles * period_height;

				println!("Eureka: {}, {}, {}, {}, {}: ~{}",
					period_height,
					rock_number,
					period,
					remaining_rocks,
					skipped_cycles,
					predicted_height,
				);

				rock_number += skipped_cycles * period;
				extra_height_prediction = Some(predicted_height);
			} else {
				rock_memories[rock_index].insert(pattern_index, Snapshot { height: grid_height, rock_number, snapshot });
			}
		}
		
		loop {
			jet_pattern[pattern_index].push(&grid, &mut rock, height);
			pattern_index = (pattern_index + 1) % jet_pattern.len();
			if height == 0 || test_collision(&grid, &rock, height - 1) {
				break
			}
			height -= 1;
		}

		for i in 0..4 {
			if rock[i] > 0 {
				if let Some(row) = grid.get_mut(height + i) {
					*row |= rock[i];
				} else {
					grid.push(rock[i]);
				}
			}
		}

		rock_number += 1;
	}

	//println!();
	//for row in grid.iter().rev() {
	//	print!("|");
	//	for i in 0..7 {
	//		if (row & 0b01000000 >> i) > 0 {
	//			print!("#");
	//		} else {
	//			print!(".");
	//		}
	//	}
	//	print!("|");
	//	println!();
	//}
	//println!("+-------+");

	println!("{}", grid.len() + extra_height_prediction.unwrap_or(0));
}