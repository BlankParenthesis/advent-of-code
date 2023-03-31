pub fn parse_input(input: &str) -> Result<Vec<usize>, ()> {
	input
		.split(',')
		.map(|s| s.parse::<usize>())
		.collect::<Result<_, _>>()
		.map_err(|_| ())
}

const RUNS: usize = 80;
const NEW_TIMER: usize = 8;
const RESET_TIMER: usize = 6;

pub fn solve(mut input: Vec<usize>) -> usize {
	for _ in 0..RUNS {
		let mut new_fish = Vec::new();
		
		for fish in input.iter_mut() {
			if *fish == 0 {
				new_fish.push(NEW_TIMER);
				*fish = RESET_TIMER;
			} else {
				*fish -= 1;
			}
		}

		input.append(&mut new_fish);
	}

	input.len()
}