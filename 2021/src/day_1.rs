pub fn parse_input(input: &str) -> Result<Vec<usize>, ()> {
	input
		.split('\n')
		.map(str::parse::<usize>)
		.collect::<Result<_, _>>()
		.map_err(|_| ())
}

pub fn solve(input: Vec<usize>) -> usize {
	input
		.windows(2)
		.filter(|window| window[1] > window[0])
		.count()
}