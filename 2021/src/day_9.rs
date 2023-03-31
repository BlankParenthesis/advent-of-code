use std::rc::Rc;

fn fast_single_digit_parse(digit: &u8) -> Result<usize, ()> {
	match digit {
		b'0' => Ok(0),
		b'1' => Ok(1),
		b'2' => Ok(2),
		b'3' => Ok(3),
		b'4' => Ok(4),
		b'5' => Ok(5),
		b'6' => Ok(6),
		b'7' => Ok(7),
		b'8' => Ok(8),
		b'9' => Ok(9),
		_ => Err(()),
	}
}

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
	let input = Rc::new(input);
	let height = input.len();
	let width = input.first().map(Vec::len).unwrap_or(0);

	(0..width).map(|x| {
		let input = Rc::clone(&input);
		(0..height).filter_map(move |y| {
			let here = input[y][x];
			let up = y.checked_sub(1).map(|y| input[y][x]);
			let down = input.get(y + 1).map(|v| v[x]);
			let left = x.checked_sub(1).map(|x| input[y][x]);
			let right = input[y].get(x + 1).copied();

			[up, down, left, right].iter()
				.all(|option| {
					option
						.map(|value| here < value)
						.unwrap_or(true)
				})
				.then(|| here)
		})
	})
	.flatten()
	.map(|point| 1 + point)
	.sum()
}