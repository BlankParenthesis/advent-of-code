use std::ops::RangeInclusive;

use crate::Part;

fn parse_range(range: &str) -> RangeInclusive<usize> {
	let (start, end) = range.split_once('-').unwrap();
	let start = start.parse::<usize>().unwrap();
	let end = end.parse::<usize>().unwrap();
	start..=end
}

pub(crate) fn solve(data: &[u8], part: Part) {
	let pairs = std::str::from_utf8(data).unwrap()
		.split('\n')
		.map(|pair| {
			let (a, b) = pair.split_once(',').unwrap();
			(parse_range(a), parse_range(b))
		})
		.collect::<Vec<(RangeInclusive<usize>, RangeInclusive<usize>)>>();
	

	match part {
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
}