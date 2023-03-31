use nom::{multi::separated_list0, bytes::complete::tag, sequence::tuple};

use crate::{Part, take_positive_number};

#[derive(Debug, Clone)]
enum Tile {
	Empty,
	Stone,
	Sand,
}

impl std::fmt::Display for Tile {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Empty => write!(f, "."),
			Self::Stone => write!(f, "#"),
			Self::Sand => write!(f, "o"),
		}
	}
}

fn draw_grid(grid: &Vec<Vec<Tile>>, width: usize, height: usize) {
	for y in 0..height {
		for x in 0..width {
			print!("{}", grid[y][x]);
		}
		println!();
	}
}

fn parse_path<Num>(input: &str) -> nom::IResult<&str, Vec<(Num, Num)>>
where Num: std::str::FromStr {
	separated_list0(tag(" -> "), tuple((take_positive_number, tag(","), take_positive_number)))(input)
		.map(|(s, vec)| {
			(s, vec.into_iter()
				.map(|(x, _, y)| (x, y))
				.collect())
		})
}

pub(crate) fn solve(data: &[u8], part: Part) {	

	let paths = std::str::from_utf8(data).unwrap().split('\n')
		.map(parse_path)
		.map(Result::unwrap)
		.map(|(_, path)| path)
		.collect::<Vec<Vec<(usize, usize)>>>();

	const SAND_INPOINT: (usize, usize) = (500, 0);

	let mut min_x = *paths.iter().flatten().chain([&SAND_INPOINT]).map(|(x, _)| x).min().unwrap();
	let min_y = *paths.iter().flatten().chain([&SAND_INPOINT]).map(|(_, y)| y).min().unwrap();
	let mut max_x = *paths.iter().flatten().chain([&SAND_INPOINT]).map(|(x, _)| x).max().unwrap();
	let max_y = *paths.iter().flatten().chain([&SAND_INPOINT]).map(|(_, y)| y).max().unwrap();

	let extra_height = match part {
		Part::A => 0,
		Part::B => 1,
	};
	
	let height = max_y - min_y + 1 + extra_height;

	if matches!(part, Part::B) {
		min_x = SAND_INPOINT.0 - height;
		max_x = SAND_INPOINT.0 + height;
	}

	let width = max_x - min_x + 1;

	let mut grid = vec![vec![Tile::Empty; width]; height];

	for path in paths {
		for pair in path.windows(2) {
			match pair {
				[(x1, y1), (x2, y2)] => {
					if x1 == x2 {
						for y in *y1.min(y2)..=*y1.max(y2) {
							grid[y - min_y][*x1 - min_x] = Tile::Stone;
						}
					} else if y1 == y2 {
						for x in *x1.min(x2)..=*x1.max(x2) {
							grid[*y1 - min_y][x - min_x] = Tile::Stone;
						}
					} else {
						unimplemented!();
					}
				},
				_ => panic!(),
			}
			
		}
	}

	const SAND_MOTION: [(isize, isize); 3] = [(0, 1), (-1, 1), (1, 1)];
	
	let mut sands = 0;
	let mut falling = true;
	
	let initial_point = (
		(SAND_INPOINT.0 - min_x) as isize,
		(SAND_INPOINT.1 - min_y) as isize
	);

	while falling {
		sands += 1;

		let mut pos = initial_point;

		while let Some(next_pos) = SAND_MOTION.iter().find_map(|(x, y)| {
			if (pos.1 + y) >= 0 && pos.0 + x >= 0 {
				let next_pos = (pos.0 + x, pos.1 + y);
				
				let tile = grid.get(next_pos.1 as usize)
					.and_then(|row| row.get(next_pos.0 as usize));

				match tile {
					None => {
						if matches!(part, Part::A) {
							falling = false;
						}
						None
					},
					Some(Tile::Empty) => {
						Some(next_pos)
					},
					_ => None,
				}
			} else {
				falling = false;
				match part {
					Part::A => None,
					Part::B => unimplemented!("requires larger grid"),
				}
			}
		}) {
			pos = next_pos;
		}

		if pos == initial_point {
			falling = false;
		}

		grid[pos.1 as usize][pos.0 as usize] = Tile::Sand;
	}
	
	draw_grid(&grid, width, height);

	match part {
		Part::A => println!("{}", sands - 1),
		Part::B => println!("{}", sands),
	}		
}