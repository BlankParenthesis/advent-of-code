use nom::{sequence::{preceded, tuple}, bytes::complete::tag};

use crate::{Part, take_positive_number};

pub(crate) fn solve(data: &[u8], part: Part) {
	let (arrangement, instructions) = std::str::from_utf8(&data).unwrap()
		.split_once("\n\n").unwrap();

		struct Instruction {
		source: usize,
		destination: usize,
		count: usize,
	}

	let instructions = instructions.split('\n').map(|i| {
		let (_, (count, source, destination)) = tuple((
			preceded(tag("move "), take_positive_number),
			preceded(tag(" from "), take_positive_number),
			preceded(tag(" to "), take_positive_number),
		))(i).unwrap();

		Instruction { source, destination, count }
	});

	let mut arrangement = arrangement.split('\n').rev();
	let labels = arrangement.next().unwrap()
		.split_whitespace()
		.filter(|n| !n.is_empty())
		.map(|n| n.parse::<usize>().unwrap())
		.collect::<Vec<usize>>();

	let mut stacks = (0..labels.len())
		.map(|_| std::collections::VecDeque::<char>::new())
		.collect::<Vec<_>>();

	for line in arrangement {
		for stack in 0..labels.len() {
			let pos = stack * 4 + 1;
			let c = line.as_bytes()[pos] as char;
			if c != ' ' {
				stacks[stack].push_back(c)
			}
		}
	}


	for Instruction { source, destination: dest, count }  in instructions {
		let source = labels.iter().position(|i| *i == source).unwrap();
		let dest = labels.iter().position(|i| *i == dest).unwrap();

		match part {
			Part::A => {
				for _ in 0..count {
					let value = stacks[source].pop_back().unwrap();
					stacks[dest].push_back(value);
				}
			},
			Part::B => {
				let mut buffer = std::collections::VecDeque::new();
				for _ in 0..count {
					let value = stacks[source].pop_back().unwrap();
					buffer.push_front(value);
				}
				stacks[dest].append(&mut buffer);
			},
		}
	}

	
	let message = stacks.iter()
		.map(|stack| stack.back().unwrap_or(&' '))
		.collect::<String>();
	println!("{}", message);
}