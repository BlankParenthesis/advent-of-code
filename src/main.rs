use std::{path::PathBuf, str::Utf8Error};

use clap::{Parser, builder::PossibleValue};

mod day_1;

pub(crate) trait Input: Sized {
	type Error;

	fn parse(data: &[u8]) -> Result<Self, Self::Error>
	where Self::Error: From<Utf8Error> {
		let string = std::str::from_utf8(data).map_err(Into::into)?;
		<Self as Input>::parse_str(string)
	}

	fn parse_str(data: &str) -> Result<Self, Self::Error>;
}

pub(crate) trait Solution {
	type Input: Input;
	type Output;

	fn solve(input: &Self::Input) -> Self::Output;
}

#[derive(Debug, Clone, clap::ValueEnum)]
enum Part {
	A,
	B,
}

#[derive(Debug, Clone)]
enum Day {
	One,
}

impl clap::ValueEnum for Day {
	fn value_variants<'a>() -> &'a [Self] {
		&[
			Day::One,
		]
	}

	fn to_possible_value(&self) -> Option<PossibleValue> {
		match self {
			Day::One => Some(PossibleValue::new("1").aliases(&["One", "1st", "first"]))
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

	match (args.day, args.part) {
		(Day::One, Part::A) => {
			let input = day_1::CalorieList::parse(&data).expect("input parse error");
			let solution = day_1::TopCalories::solve(&input);
			println!("{}", solution);
		},
		(Day::One, Part::B) => {
			let input = day_1::CalorieList::parse(&data).expect("input parse error");
			let solution = day_1::Top3Calories::solve(&input);
			println!("{:?}", solution);
		},
	}
}
