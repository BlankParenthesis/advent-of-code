use std::{str::Utf8Error, num::ParseIntError};

use crate::{Input, Solution};

#[derive(Debug)]
pub(crate) enum CalorieListParseError {
	Utf8Error(Utf8Error),
	InvalidCalorie(ParseIntError),
	EmptyList,
}

impl From<Utf8Error> for CalorieListParseError {
    fn from(err: Utf8Error) -> Self {
        Self::Utf8Error(err)
    }
}

#[derive(Debug)]
struct Elf {
	calories: Vec<usize>,
}

#[derive(Debug)]
pub(crate) struct CalorieList {
	elves: Vec<Elf>,
}

impl Input for CalorieList {
    type Error = CalorieListParseError;

    fn parse_str(data: &str) -> Result<Self, Self::Error> {
		data.trim()
			.split("\n\n")
			.map(|slice| {
				slice.split('\n')
					.map(|cal| cal.trim().parse::<usize>())
					.collect::<Result<_, _>>()
					.map(|calories| Elf { calories })
					.map_err(CalorieListParseError::InvalidCalorie)
			})
			.collect::<Result<_, _>>()
			.map(|elves| Self { elves })
			.and_then(|list| {
				if list.elves.is_empty() {
					Err(CalorieListParseError::EmptyList)
				} else {
					Ok(list)
				}
			})
    }
}

pub(crate) struct TopCalories {}

impl Solution for TopCalories {
	type Input = CalorieList;
	type Output = usize;

	fn solve(input: &Self::Input) -> Self::Output {
        input.elves.iter()
			.map(|elf| elf.calories.iter().sum())
			.max()
			.unwrap()
    }

}
pub(crate) struct Top3Calories {}

impl Solution for Top3Calories {
	type Input = CalorieList;
	type Output = usize;

	fn solve(input: &Self::Input) -> Self::Output {
        let mut sums = input.elves.iter()
			.map(|elf| elf.calories.iter().sum())
			.collect::<Vec<usize>>();

		sums.sort_unstable_by(|a, b| b.cmp(a));

		assert!(sums.len() >= 3);
		
		sums.iter().take(3).sum()
    }

}
