use std::collections::HashSet;

use crate::Part;

pub(crate) fn solve(data: &[u8], part: Part) {
	let input = std::str::from_utf8(data).expect("input parse error");

	enum Direction {
		Right,
		Left,
		Down,
		Up,
	}

	struct Movement {
		direction: Direction,
		count: isize,
	}

	impl TryFrom<char> for Direction {
		type Error = ();

		fn try_from(c: char) -> Result<Self, Self::Error> {
			match c {
				'R' => Ok(Self::Right),
				'L' => Ok(Self::Left),
				'D' => Ok(Self::Down),
				'U' => Ok(Self::Up),
				_ => Err(()),
			}
		}
	}

	let instructions = input.split('\n')
		.map(|line| {
			let (d, c) = line.split_once(' ').unwrap();
			let direction = Direction::try_from(d.chars().next().unwrap()).unwrap();
			let count = c.parse::<isize>().unwrap();

			Movement { direction, count }
		})
		.collect::<Vec<_>>();

	let rope_length = match part {
		Part::A => 2,
		Part::B => 10,
	};

	let mut visited_positions: HashSet<(isize, isize)> = HashSet::new();
	let mut positions = vec![(0, 0); rope_length];
	
	visited_positions.insert(*positions.last().unwrap());

	for instruction in instructions {
		for _ in 0..instruction.count {
			match instruction.direction {
				Direction::Down => positions[0].1 += 1,
				Direction::Left => positions[0].0 -= 1,
				Direction::Right => positions[0].0 += 1,
				Direction::Up => positions[0].1 -= 1,
			}

			for i in 1..positions.len() {
				let previous = positions[i - 1];
				let current = &mut positions[i];
				
				let diff_x = isize::abs_diff(previous.0, current.0);
				let diff_y = isize::abs_diff(previous.1, current.1);

				if diff_x > 1 || diff_y > 1 {
					current.0 += (previous.0 - current.0).clamp(-1, 1);
					current.1 += (previous.1 - current.1).clamp(-1, 1);
				}
			}

			visited_positions.insert(*positions.last().unwrap());
		}
	}

	println!("{}", visited_positions.len());
}