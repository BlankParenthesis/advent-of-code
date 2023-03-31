use crate::Part;

pub(crate) fn solve(data: &[u8], part: Part) {	
	let grid = std::str::from_utf8(data).unwrap()
		.split('\n')
		.map(|line| {
			line.chars()
				.map(|c| {
					assert!(c.is_ascii_digit());
					(c as u8 - b'0') as i8
				})
				.collect::<Vec<_>>()
		})
		.collect::<Vec<_>>();

	match part {
		Part::A => {
			let mut visiblity = grid.iter()
				.map(|c| vec![false; c.len()])
				.collect::<Vec<_>>();

			// horizontal

			for (y, line) in grid.iter().enumerate() {
				let mut best = -1;
				for (x, value) in line.iter().enumerate() {
					if *value > best {
						best = *value;
						visiblity[y][x] = true;
					}
				}
			}

			for (y, line) in grid.iter().enumerate() {
				let mut best = -1;
				for (x, value) in line.iter().enumerate().rev() {
					if *value > best {
						best = *value;
						visiblity[y][x] = true;
					}
				}
			}

			// vertical
			let mut bests = vec![-1; grid[0].len()];

			for (y, line) in grid.iter().enumerate() {
				for (x, value) in line.iter().enumerate() {
					if *value > bests[x] {
						bests[x] = *value;
						visiblity[y][x] = true;
					}
				}
			}

			let mut bests = vec![-1; grid[0].len()];

			for (y, line) in grid.iter().enumerate().rev() {
				for (x, value) in line.iter().enumerate() {
					if *value > bests[x] {
						bests[x] = *value;
						visiblity[y][x] = true;
					}
				}
			}

			println!("{:?}", visiblity.iter().flatten().filter(|a| **a).count());
		},
		Part::B => {
			let mut scenic_score = grid.iter()
				.map(|c| vec![0; c.len()])
				.collect::<Vec<_>>();

			let height = grid.len();
			let width = grid[0].len();

			for (y, row) in grid.iter().enumerate().skip(1).take(height - 2) {
				for (x, value) in row.iter().enumerate().skip(1).take(width - 2) {
					let value = *value;

					let mut left_score = 0;
					for nx in (0..x).rev() {
						left_score += 1;
						if row[nx] >= value {
							break;
						}
					}

					let mut right_score = 0;
					for nx in (x..width).skip(1) {
						right_score += 1;
						if row[nx] >= value {
							break;
						}
					}

					let mut up_score = 0;
					for ny in (0..y).rev() {
						up_score += 1;
						if grid[ny][x] >= value {
							break;
						}
					}

					let mut down_score = 0;
					for ny in (y..height).skip(1) {
						down_score += 1;
						if grid[ny][x] >= value {
							break;
						}
					}

					scenic_score[y][x] = left_score * up_score * right_score * down_score;
				}
			}
			println!("{:?}", scenic_score.iter().flatten().max().unwrap());
		},
	};
}