use enumset::{EnumSetType, EnumSet};
use std::convert::TryFrom;

#[derive(Debug, EnumSetType)]
pub enum Segment {
	A,
	B,
	C,
	D,
	E,
	F,
	G,
}

impl TryFrom<u8> for Segment {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
			b'a' => Ok(Segment::A),
			b'b' => Ok(Segment::B),
			b'c' => Ok(Segment::C),
			b'd' => Ok(Segment::D),
			b'e' => Ok(Segment::E),
			b'f' => Ok(Segment::F),
			b'g' => Ok(Segment::G),
			_ => Err(()),
		}
    }
}

fn parse_segments(segments: &str) -> Result<EnumSet<Segment>, ()> {
	segments
		.as_bytes()
		.iter()
		.map(|byte| Segment::try_from(*byte))
		.collect()
}

pub fn parse_input(input: &str) -> Result<Vec<[Vec<EnumSet<Segment>>; 2]>, ()> {
	input
		.split('\n')
		.map(|line| {
			let (patterns, output) = line.split_once(" | ").ok_or(())?;
			Ok([
				patterns.split(' ').map(parse_segments).collect::<Result<Vec<_>, _>>()?,
				output.split(' ').map(parse_segments).collect::<Result<Vec<_>, _>>()?,
			])
		})
		.collect()
}

// number → segment count
// 0: 6
// 1: 2
// 2: 5
// 3: 5
// 4: 4
// 5: 5
// 6: 6
// 7: 3
// 8: 7
// 9: 6

//segment count → numbers
// 2: [1]
// 3: [7]
// 4: [4]
// 5: [2,3,5]
// 6: [0,6,9]
// 7: [8]

// eh~, ez-modo~?
// hard mode is obviously working out all the mappings
// it looks possible at first glance but it wasn't what was asked 🤷
pub fn solve(input: Vec<[Vec<EnumSet<Segment>>; 2]>) -> usize {
	input.into_iter().map(|[_, outputs]| {
		outputs.into_iter().filter(|output| {
			matches!(output.len(), 2 | 3 | 4 | 7)
		}).count()
	})
	.sum()
}