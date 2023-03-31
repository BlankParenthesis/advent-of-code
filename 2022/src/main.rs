use std::{path::PathBuf, str::Utf8Error, ops::RangeInclusive, collections::{HashSet, HashMap, hash_map::Entry}, cmp::Ordering};

use clap::{Parser, builder::PossibleValue};
use nom::{bytes::complete::tag, sequence::{preceded, tuple, delimited}, combinator::{map_res, map}, character::complete::{digit1, alpha1}, multi::separated_list0, Parser as NomParser};
use petgraph::prelude::NodeIndex;

mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;
mod day_9;
mod day_10;
mod day_11;
mod day_12;
mod day_13;
mod day_14;
mod day_15;
mod day_16;
mod day_17;
mod day_18;
mod day_19;

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
	Nine,
	Ten,
	Eleven,
	Twelve,
	Thirteen,
	Fourteen,
	Fifteen,
	Sixteen,
	Seventeen,
	Eighteen,
	Nineteen,
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
			Day::Nine,
			Day::Ten,
			Day::Eleven,
			Day::Twelve,
			Day::Thirteen,
			Day::Fourteen,
			Day::Fifteen,
			Day::Sixteen,
			Day::Seventeen,
			Day::Eighteen,
			Day::Nineteen,
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
			Day::Nine => Some(PossibleValue::new("9").aliases(&["nine", "9th", "ninth"])),
			Day::Ten => Some(PossibleValue::new("10").aliases(&["ten", "10th", "tenth"])),
			Day::Eleven => Some(PossibleValue::new("11").aliases(&["eleven", "11th", "eleventh"])),
			Day::Twelve => Some(PossibleValue::new("12").aliases(&["twelve", "12th", "twelfth"])),
			Day::Thirteen => Some(PossibleValue::new("13").aliases(&["thirteen", "13th", "thirteenth"])),
			Day::Fourteen => Some(PossibleValue::new("14").aliases(&["fourteen", "14th", "fourteenth"])),
			Day::Fifteen => Some(PossibleValue::new("15").aliases(&["fifteen", "15th", "fifteenth"])),
			Day::Sixteen => Some(PossibleValue::new("16").aliases(&["sixteen", "16th", "sixteenth"])),
			Day::Seventeen => Some(PossibleValue::new("17").aliases(&["seventeen", "17th", "seventeenth"])),
			Day::Eighteen => Some(PossibleValue::new("18").aliases(&["eighteen", "18th", "eighteenth"])),
			Day::Nineteen => Some(PossibleValue::new("19").aliases(&["nineteen", "19th", "nineteenth"])),
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

pub fn take_positive_number<Num>(input: &str) -> nom::IResult<&str, Num>
where Num: std::str::FromStr {
	map_res(digit1, str::parse)(input)
}

pub fn take_negative_number<Num>(input: &str) -> nom::IResult<&str, Num>
where Num: std::str::FromStr {
	preceded(tag("-"), map_res(digit1, |d: &str| str::parse(("-".to_owned() + d).as_str())))(input)
}

pub fn take_number<Num>(input: &str) -> nom::IResult<&str, Num>
where Num: std::str::FromStr {
	take_positive_number(input)
	.or_else(|_| take_negative_number(input))
}

fn main() {
	let args = Args::parse();

	let data = std::fs::read(&args.input_path).expect("invalid path");

	let is_example = args.input_path.file_name().unwrap()
		.to_str().unwrap()
		.contains("example");

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
			day_4::solve(&data, args.part);
		},
		(Day::Five, _) => {
			day_5::solve(&data, args.part);
		},
		(Day::Six, _) => {
			day_6::solve(&data, args.part);
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
			day_8::solve(&data, args.part);
		},
		(Day::Nine, _) => {
			day_9::solve(&data, args.part);
		},
		(Day::Ten, _) => {
			day_10::solve(&data, args.part);
		},
		(Day::Eleven, _) => {
			day_11::solve(&data, args.part);
		},
		(Day::Twelve, _) => {
			day_12::solve(&data, args.part);
		},
		(Day::Thirteen, _) => {
			day_13::solve(&data, args.part);
		},
		(Day::Fourteen, _) => {		
			day_14::solve(&data, args.part);
		},
		(Day::Fifteen, _) => {
			day_15::solve(&data, args.part, is_example);
		},
		(Day::Sixteen, _) => {
			day_16::solve(&data, args.part);
		},
		(Day::Seventeen, _) => {
			day_17::solve(&data, args.part);
		},
		(Day::Eighteen, _) => {
			day_18::solve(&data, args.part);
		},
		(Day::Nineteen, _) => {
			unimplemented!("Day 19 unsolved");

			let blueprints = day_19::Blueprint::parse_all(std::str::from_utf8(&data).unwrap()).unwrap().1;

			const TIME_AVAILABLE: isize = 24;

			let quality_sum = blueprints.iter()
				//.map(|b| b.id * b.find_max_geodes(None, TIME_AVAILABLE))
				//.sum::<usize>();
				.take(1)
				.map(|b| b.find_max_geodes(TIME_AVAILABLE))
				.collect::<Vec<_>>();
			println!("{:?}", quality_sum);
		},
	}
}
