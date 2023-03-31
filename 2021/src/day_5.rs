use geo::{Line, algorithm::line_intersection::line_intersection};
use itertools::Itertools;

pub fn parse_input(input: &str) -> Result<Vec<Line<f32>>, ()> {
	input.split('\n').map(|line| {
		let (start, end) = line.split_once(" -> ").ok_or(())?;

		let (x1, y1) = start.split_once(',').ok_or(())?;
		let (x2, y2) = end.split_once(',').ok_or(())?;

		let x1 = x1.parse::<f32>().map_err(|_| ())?;
		let y1 = y1.parse::<f32>().map_err(|_| ())?;
		let x2 = x2.parse::<f32>().map_err(|_| ())?;
		let y2 = y2.parse::<f32>().map_err(|_| ())?;

		Ok(Line {
			start: (x1, y1).into(),
			end: (x2, y2).into()
		})
	}).collect()
}

pub fn solve(input: Vec<Line<f32>>) -> usize {
	input
		.iter()
		.combinations(2)
		.filter(|pair| {
			let a = **pair.first().unwrap();
			let b = **pair.last().unwrap();

			line_intersection(a, b).is_some()
		})
		.count()
}