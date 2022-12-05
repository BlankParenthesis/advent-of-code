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
}

impl clap::ValueEnum for Day {
	fn value_variants<'a>() -> &'a [Self] {
		&[
			Day::One,
			Day::Two,
			Day::Three,
			Day::Four,
		]
	}

	fn to_possible_value(&self) -> Option<PossibleValue> {
		match self {
			Day::One => Some(PossibleValue::new("1").aliases(&["one", "1st", "first"])),
			Day::Two => Some(PossibleValue::new("2").aliases(&["two", "2nd", "second"])),
			Day::Three => Some(PossibleValue::new("3").aliases(&["three", "3rd", "third"])),
			Day::Four => Some(PossibleValue::new("4").aliases(&["four", "4th", "fourth"])),
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
	}
}
