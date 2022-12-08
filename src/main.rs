use std::{path::PathBuf, str::Utf8Error, ops::RangeInclusive};

use clap::{Parser, builder::PossibleValue};
use nom::{bytes::complete::tag, sequence::{preceded, tuple}, combinator::map_res, character::complete::digit1};

mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_7;

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
	Seven,
	Eight,
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
			Day::Seven,
			Day::Eight,
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
			Day::Seven => Some(PossibleValue::new("7").aliases(&["seven", "7th", "seventh"])),
			Day::Eight => Some(PossibleValue::new("8").aliases(&["eight", "8th", "eighth"])),
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

			fn take_number(input: &str) -> nom::IResult<&str, usize> {
				map_res(digit1, str::parse)(input)
			}

			struct Instruction {
				source: usize,
				destination: usize,
				count: usize,
			}

			let instructions = instructions.split('\n').map(|i| {
				let (_, (count, source, destination)) = tuple((
					preceded(tag("move "), take_number),
					preceded(tag(" from "), take_number),
					preceded(tag(" to "), take_number),
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
		(Day::Seven, _) => {
			let tree = day_7::Directory::create_from_str(std::str::from_utf8(&data).unwrap()).unwrap();

			match args.part {
				Part::A => {
					println!("{:?}", day_7::solve_a(&tree));
				},
				Part::B => {
					println!("{:?}", day_7::solve_b(&tree));
				},
			};
		},
		(Day::Eight, _) => {
			let grid = std::str::from_utf8(&data).unwrap()
				.split('\n')
				.map(|line| {
					line.chars()
						.map(|c| {
							assert!(c.is_ascii_digit());
							(c as u8 - b'0') as i8
						})
						.collect::<Vec<_>>()
				})
				.collect::<Vec<_>>();

			match args.part {
				Part::A => {
					let mut visiblity = grid.iter()
						.map(|c| vec![false; c.len()])
						.collect::<Vec<_>>();

					// horizontal

					for (y, line) in grid.iter().enumerate() {
						let mut best = -1;
						for (x, value) in line.iter().enumerate() {
							if *value > best {
								best = *value;
								visiblity[y][x] = true;
							}
						}
					}

					for (y, line) in grid.iter().enumerate() {
						let mut best = -1;
						for (x, value) in line.iter().enumerate().rev() {
							if *value > best {
								best = *value;
								visiblity[y][x] = true;
							}
						}
					}

					// vertical
					let mut bests = vec![-1; grid[0].len()];

					for (y, line) in grid.iter().enumerate() {
						for (x, value) in line.iter().enumerate() {
							if *value > bests[x] {
								bests[x] = *value;
								visiblity[y][x] = true;
							}
						}
					}

					let mut bests = vec![-1; grid[0].len()];

					for (y, line) in grid.iter().enumerate().rev() {
						for (x, value) in line.iter().enumerate() {
							if *value > bests[x] {
								bests[x] = *value;
								visiblity[y][x] = true;
							}
						}
					}

					println!("{:?}", visiblity.iter().flatten().filter(|a| **a).count());
				},
				Part::B => {
					let mut scenic_score = grid.iter()
						.map(|c| vec![0; c.len()])
						.collect::<Vec<_>>();

					let height = grid.len();
					let width = grid[0].len();

					for (y, row) in grid.iter().enumerate().skip(1).take(height - 2) {
						for (x, value) in row.iter().enumerate().skip(1).take(width - 2) {
							let value = *value;

							let mut left_score = 0;
							for nx in (0..x).rev() {
								left_score += 1;
								if row[nx] >= value {
									break;
								}
							}

							let mut right_score = 0;
							for nx in (x..width).skip(1) {
								right_score += 1;
								if row[nx] >= value {
									break;
								}
							}

							let mut up_score = 0;
							for ny in (0..y).rev() {
								up_score += 1;
								if grid[ny][x] >= value {
									break;
								}
							}

							let mut down_score = 0;
							for ny in (y..height).skip(1) {
								down_score += 1;
								if grid[ny][x] >= value {
									break;
								}
							}

							scenic_score[y][x] = left_score * up_score * right_score * down_score;
						}
					}
					println!("{:?}", scenic_score.iter().flatten().max().unwrap());
				},
			};
		}
	}
}
