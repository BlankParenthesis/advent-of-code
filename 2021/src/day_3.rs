pub fn parse_input(input: &str) -> Result<Vec<Vec<bool>>, ()> {
	let numbers = input.split('\n');

	numbers.map(|bits| {
		bits.as_bytes().iter().map(|bit| {
			match bit {
				b'1' => Ok(true),
				b'0' => Ok(false),
				_ => Err(()),
			}
		}).collect::<Result<_, _>>()
	}).collect::<Result<_, _>>()
}

fn transpose<T>(v: Vec<Vec<T>>) -> Result<Vec<Vec<T>>, ()> {
    let len = v.first().map(Vec::len).unwrap_or(0);
    let mut iters: Vec<_> = v.into_iter().map(Vec::into_iter).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().ok_or(()))
                .collect::<Result<_, _>>()
        })
        .collect()
}


pub fn solve(input: Vec<Vec<bool>>) -> usize {
	let input = transpose(input).unwrap();

	let (gamma, epsilon) = input
		.into_iter()
		.map(|bits| {
			let majority = (bits.len() / 2) + 1;
			let true_bits = bits.into_iter().filter(|b| *b).count();
			true_bits >= majority
		})
		.rev()
		.enumerate()
		.map(|(i, bit)| ((bit as usize) << i, (!bit as usize) << i) )
		.fold((0, 0), |(sum_gamma, sum_epsilon), (next_gamma, next_epsilon)| {
			(sum_gamma | next_gamma, sum_epsilon | next_epsilon)
		});

	gamma * epsilon
}