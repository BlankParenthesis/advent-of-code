
pub fn parse_input(input: &str) -> Result<Vec<Vec<usize>>, ()> {
	input.split('\n').map(|row| {
		row
			.as_bytes()
			.into_iter()
			.map(fast_single_digit_parse)
			.collect::<Result<_, _>>()
	}).collect::<Result<_, _>>()
}

pub fn solve(input: Vec<Vec<usize>>) -> usize {
	
}