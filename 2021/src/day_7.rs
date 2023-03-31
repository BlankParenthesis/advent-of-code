pub fn parse_input(input: &str) -> Result<Vec<usize>, ()> {
	input
		.split(',')
		.map(|s| s.parse::<usize>())
		.collect::<Result<_, _>>()
		.map_err(|_| ())
}

pub fn solve(mut input: Vec<usize>) -> usize {
	input.sort_unstable();
	let ideal_pos = input[input.len() / 2];

	input
		.into_iter()
		.map(|pos| usize::max(pos, ideal_pos) - usize::min(pos, ideal_pos))
		.sum()
}