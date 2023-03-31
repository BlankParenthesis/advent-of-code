use std::convert::TryFrom;
use enum_map::{EnumMap, Enum, enum_map};

#[derive(Debug, Enum, Clone, Copy, PartialEq)]
pub enum BraceType {
	Round,
	Square,
	Curly,
	Angle,
}

impl BraceType {
	fn score(&self) -> usize {
		match self {
			BraceType::Round => 3,
			BraceType::Square => 57,
			BraceType::Curly => 1197,
			BraceType::Angle => 25137,
		}
	}
}

#[derive(Debug)]
pub enum Brace {
	Open(BraceType),
	Close(BraceType),
}

impl TryFrom<u8> for Brace {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
			b'(' => Ok(Self::Open(BraceType::Round)),
			b')' => Ok(Self::Close(BraceType::Round)),
			b'[' => Ok(Self::Open(BraceType::Square)),
			b']' => Ok(Self::Close(BraceType::Square)),
			b'{' => Ok(Self::Open(BraceType::Curly)),
			b'}' => Ok(Self::Close(BraceType::Curly)),
			b'<' => Ok(Self::Open(BraceType::Angle)),
			b'>' => Ok(Self::Close(BraceType::Angle)),
			_ => Err(()),
		}
    }
}

pub fn parse_input(input: &str) -> Result<Vec<Vec<Brace>>, ()> {
	input.split('\n')
		.map(|line| {
			line
				.as_bytes()
				.iter()
				.map(|byte| Brace::try_from(*byte))
				.collect::<Result<Vec<_>, _>>()
		})
		.collect::<Result<Vec<_>, _>>()
}

pub fn solve(input: Vec<Vec<Brace>>) -> usize {
	input.into_iter().filter_map(|line| {
		let mut stack = Vec::<BraceType>::new();

		for brace in line {
			match brace {
				Brace::Close(brace_type) => {
					match stack.pop() {
						Some(pop_type) => {
							if pop_type != brace_type {
								return Some(brace_type);
							}
						},
						None => {
							return Some(brace_type);
						}
					}
				},
				Brace::Open(brace_type) => {
					stack.push(brace_type);
				},
			}
		}

		None
	})
	.map(|b| b.score())
	.sum()
}