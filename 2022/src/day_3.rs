use std::str::Utf8Error;
use std::collections::HashSet;

use crate::Input;

#[derive(Debug)]
pub(crate) enum PackingParseError{
	Utf8Error(Utf8Error),
	OddPackageCount,
	InvalidItem(char),
}

impl From<Utf8Error> for PackingParseError {
    fn from(err: Utf8Error) -> Self {
        Self::Utf8Error(err)
    }
}

#[derive(Debug)]
pub(crate) struct Rucksack {
	items: Vec<u8>,
}

impl Rucksack {
	fn first_compartment(&self) -> &[u8] {
		self.items.split_at(self.items.len() / 2).0
	}

	fn second_compartment(&self) -> &[u8] {
		self.items.split_at(self.items.len() / 2).1
	}

	fn parse_compartment(s: &str) -> Result<Vec<u8>, PackingParseError> {
		s.chars()
			.map(|c| {
				match c {
					'a'..='z' => Ok(c as u8 - b'a' + 1),
					'A'..='Z' => Ok(c as u8 - b'A' + 27),
					_ => Err(PackingParseError::InvalidItem(c))
				}
			})
			.collect()
	}

	fn from_str(s: &str) -> Result<Self, PackingParseError> {
		if s.len() % 2 == 0 {
			Ok(Self {
				items: Self::parse_compartment(s)?,
			})
		} else {
			Err(PackingParseError::OddPackageCount)
		}
	}

	pub(crate) fn incorrect_items(&self) -> Vec<u8> {
		let a = HashSet::<u8>::from_iter(self.first_compartment().iter().copied());
		let b = HashSet::<u8>::from_iter(self.second_compartment().iter().copied());

		a.intersection(&b).copied().collect()
	}
}

#[derive(Debug)]
pub(crate) struct Packing {
	rucksacks: Vec<Rucksack>,
}

impl Input for Packing {
    type Error = PackingParseError;

    fn parse_str(data: &str) -> Result<Self, Self::Error> {
        data.split('\n')
			.map(Rucksack::from_str)
			.collect::<Result<Vec<_>, _>>()
			.map(|rucksacks| Self { rucksacks })
    }
}

#[derive(Debug)]
pub(crate) enum PackingError {
	NoMatch,
	MultipleMatches,
}

impl Packing {
	pub(crate) fn wrong_item_priority_sum(&self) -> Result<usize, PackingError> {
		self.rucksacks.iter()
			.map(|sack| match &sack.incorrect_items()[..] {
				[] => Err(PackingError::NoMatch),
				[item] => Ok(*item as usize),
				_ => Err(PackingError::MultipleMatches)
			})
			.collect::<Result<Vec<_>, _>>()
			.map(|list| list.iter().sum())
	}

	pub(crate) fn badges_priority_sum(&self) -> Result<usize, PackingError> {
		self.rucksacks.
			chunks(3)
			.map(|group| {
				group.iter()
					.map(|rucksack| {
						HashSet::<u8>::from_iter(rucksack.items.iter().copied())
					})
					.reduce(|a, b| a.intersection(&b).copied().collect())
					.unwrap()
			}) 
			.map(|badges| badges.iter().copied().collect::<Vec<_>>())
			.map(|badges| match &badges[..] {
				[] => Err(PackingError::NoMatch),
				[item] => Ok(*item as usize),
				_ => Err(PackingError::MultipleMatches)
			})
			.collect::<Result<Vec<_>, _>>()
			.map(|list| list.iter().sum())
	}
}