use crate::Part;

pub(crate) fn solve(data: &[u8], part: Part) {
	let size = match part {
		Part::A => 4,
		Part::B => 14,
	};

	let index = data.windows(size)
		.position(|window| {
			let mut bitset: usize = 0;
			for byte in window {
				let byte_index = byte - b'a';
				bitset |= 1 << byte_index;
			}

			(bitset.count_ones() as usize) == size
		})
		.unwrap();

	println!("{}", index + size);
}