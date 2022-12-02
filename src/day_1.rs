use std::{str::Utf8Error, num::ParseIntError};

use crate::Input;

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

impl Elf {
	fn total(&self) -> usize {
		self.calories.iter().sum()
	}
}

impl TryFrom<&str> for Elf {
	type Error = CalorieListParseError;

	fn try_from(input: &str) -> Result<Self, Self::Error> {
		input.split('\n')
			.map(|cal| cal.trim().parse::<usize>())
			.collect::<Result<_, _>>()
			.map(|calories| Self { calories })
			.map_err(CalorieListParseError::InvalidCalorie)
	}
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
			.map(Elf::try_from)
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

impl CalorieList {
	pub fn top(&self) -> usize {
        self.elves
			.iter()
			.map(Elf::total)
			.max()
			.unwrap()
    }

	pub fn top_n(&self, n: usize) -> usize {
        let mut sums = self.elves
			.iter()
			.map(Elf::total)
			.collect::<Vec<usize>>();

		sums.sort_unstable_by(|a, b| b.cmp(a));

		assert!(sums.len() >= n);
		
		sums.iter().take(n).sum()
    }
}
