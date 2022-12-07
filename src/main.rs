use std::{path::PathBuf, str::Utf8Error, ops::RangeInclusive};

use clap::{Parser, builder::PossibleValue};

mod day_1;
mod day_2;
mod day_3;
mod day_4;

pub(crate) trait Input: Sized {
	type Error;

	fn parse(data: &[u8]) -> Result<Self, Self::Error>
	where Self::Error: From<Utf8Error> {
		let string = std::str::from_utf8(data).map_err(Into::into)?;
		<Self as Input>::parse_str(string)
	}

	fn parse_str(data: &str) -> Result<Self, Self::Error>;
}

#[derive(Debug, Clone, clap::ValueEnum)]
enum Part {
	A,
	B,
}

#[derive(Debug, Clone)]
enum Day {
	One,
	Two,
	Three,
	Four,
	Five,
	Six,
}

impl clap::ValueEnum for Day {
	fn value_variants<'a>() -> &'a [Self] {
		&[
			Day::One,
			Day::Two,
			Day::Three,
			Day::Four,
			Day::Five,
			Day::Six,
		]
	}

	fn to_possible_value(&self) -> Option<PossibleValue> {
		match self {
			Day::One => Some(PossibleValue::new("1").aliases(&["one", "1st", "first"])),
			Day::Two => Some(PossibleValue::new("2").aliases(&["two", "2nd", "second"])),
			Day::Three => Some(PossibleValue::new("3").aliases(&["three", "3rd", "third"])),
			Day::Four => Some(PossibleValue::new("4").aliases(&["four", "4th", "fourth"])),
			Day::Five => Some(PossibleValue::new("5").aliases(&["five", "5th", "fifth"])),
			Day::Six => Some(PossibleValue::new("6").aliases(&["six", "6th", "sixth"])),
		}
	}
}

#[derive(Parser, Debug)]
struct Args {
	#[arg(value_enum)]
	day: Day,
	#[arg(value_enum)]
	part: Part,
	input_path: PathBuf,
}

fn main() {
	let args = Args::parse();

	let data = std::fs::read(args.input_path).expect("invalid path");

	match (args.day, &args.part) {
		(Day::One, Part::A) => {
			let input = day_1::CalorieList::parse(&data).expect("input parse error");
			println!("{}", input.top());
		},
		(Day::One, Part::B) => {
			let input = day_1::CalorieList::parse(&data).expect("input parse error");
			println!("{:?}", input.top_n(3));
		},
		(Day::Two, Part::A) => {
			let input = day_2::ActionStrategyGuide::parse(&data).expect("input parse error");
			println!("{}", input.score());
		},
		(Day::Two, Part::B) => {
			let input = day_2::OutcomeStrategyGuide::parse(&data).expect("input parse error");
			println!("{}", input.score());
		},
		(Day::Three, Part::A) => {
			let input = day_3::Packing::parse(&data).expect("input parse error");
			println!("{}", input.wrong_item_priority_sum().expect("solve error"));
		},
		(Day::Three, Part::B) => {
			let input = day_3::Packing::parse(&data).expect("input parse error");
			println!("{}", input.badges_priority_sum().expect("solve error"));
		},
		(Day::Four, _) => {
			fn parse_range(range: &str) -> RangeInclusive<usize> {
				let (start, end) = range.split_once('-').unwrap();
				let start = start.parse::<usize>().unwrap();
				let end = end.parse::<usize>().unwrap();
				start..=end
			}

			let pairs = std::str::from_utf8(&data).unwrap()
				.split('\n')
				.map(|pair| {
					let (a, b) = pair.split_once(',').unwrap();
					(parse_range(a), parse_range(b))
				})
				.collect::<Vec<(RangeInclusive<usize>, RangeInclusive<usize>)>>();
			

			match args.part {
				Part::A => {
					let overlapping = pairs.iter().filter(|(a, b)| {
						b.clone().step_by(1).all(|v| a.contains(&v)) ||
						a.clone().step_by(1).all(|v| b.contains(&v))
					});
		
					println!("{:?}", overlapping.count())
				},
				Part::B => {
					let overlapping = pairs.iter().filter(|(a, b)| {
						b.clone().step_by(1).any(|v| a.contains(&v))
					});
		
					println!("{:?}", overlapping.count())
				},
			}
		},
		(Day::Five, _) => {
			let (arrangement, instructions) = std::str::from_utf8(&data).unwrap()
				.split_once("\n\n").unwrap();

			struct Instruction {
				source: usize,
				dest: usize,
				count: usize,
			}

			let instructions = instructions.split('\n').map(|i| {
				let (count, locations) = i[5..].split_once(" from ").unwrap();
				let (source, dest) = locations.split_once(" to ").unwrap();
				let count = count.parse().unwrap();
				let source = source.parse().unwrap();
				let dest = dest.parse().unwrap();
				Instruction { source, dest, count }
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


			for Instruction { source, dest, count }  in instructions {
				let source = labels.iter().position(|i| *i == source).unwrap();
				let dest = labels.iter().position(|i| *i == dest).unwrap();

				match args.part {
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
		},
		(Day::Six, _) => {
			let size = match args.part {
				Part::A => 4,
				Part::B => 14,
			};

			let index = data.windows(size)
				.position(|window| {
					let mut bitset: usize = 0;
					for byte in window {
						let byte_index = byte - b'a';
						bitset |= 1 << byte_index;
					}

					(bitset.count_ones() as usize) == size
				})
				.unwrap();

			println!("{}", index + size);
		},
	}
}
