use std::collections::HashSet;

use nom::{bytes::complete::tag, sequence::tuple};

use crate::{Part, take_positive_number};

struct Grid {
	width: u32,
	height: u32,
	depth: u32,
	voxels: HashSet<u128>,
}

impl Grid {
	fn new(width: u32, height: u32, depth: u32) -> Self {
		Self {
			width,
			height,
			depth,
			voxels: HashSet::new(),
		}
	}

	fn address(&self, x: u32, y: u32, z: u32) -> u128 {
		x as u128
		| (y as u128) << 32
		| (z as u128) << 64
	} 

	fn unaddress(&self, address: u128) -> (u32, u32, u32) {
		(
			(address & 0xffffffff) as u32,
			((address >> 32) & 0xffffffff) as u32,
			((address >> 64) & 0xffffffff) as u32,
		)
	}

	fn adjacent(&self, x: u32, y: u32, z: u32) -> [Option<(u32, u32, u32)>; 6] {
		[
			(x > 0).then_some((x - 1, y, z)),
			(x < self.height - 1).then_some((x + 1, y, z)),
			(y > 0).then_some((x, y - 1, z)),
			(y < self.width - 1).then_some((x, y + 1, z)),
			(z > 0).then_some((x, y, z - 1)),
			(z < self.depth - 1).then_some((x, y, z + 1)),
		]
	}

	fn adjacent_count(&self, x: u32, y: u32, z: u32) -> usize {
		self.adjacent(x, y, z)
			.into_iter()
			.flatten()
			.filter(|(x, y, z)| {
				self.voxels.contains(&self.address(*x, *y, *z))
			})
			.count()
	}

	fn set(&mut self, x: u32, y: u32, z: u32) {
		self.voxels.insert(self.address(x, y, z));
	}
}

pub(crate) fn solve(data: &[u8], part: Part) {
	let positions = std::str::from_utf8(data).unwrap().split('\n')
		.map(|l| {
			let (_, (x, _, y, _, z)) = tuple((
				take_positive_number::<u32>, tag(","),
				take_positive_number::<u32>, tag(","),
				take_positive_number::<u32>
			))(l).unwrap();

			(x + 1, y + 1, z + 1)
		})
		.collect::<Vec<(u32, u32, u32)>>();

	let width = *positions.iter().map(|(x, _, _)| x).max().unwrap() + 2;
	let height = *positions.iter().map(|(_, y, _)| y).max().unwrap() + 2;
	let depth = *positions.iter().map(|(_, _, z)| z).max().unwrap() + 2;

	let mut grid = Grid::new(width, height, depth);

	let mut sides = 0;


	for (x, y, z) in positions {
		sides += 6 - 2 * grid.adjacent_count(x, y, z) as i32;
		grid.set(x, y, z);
	}

	match part {
		Part::A => println!("{}", sides),
		Part::B => {
			let mut visited_tracker = Grid::new(width, height, depth);
			let mut visiting_tracker = Grid::new(width, height, depth);

			visiting_tracker.set(0, 0, 0);

			let mut faces = 0;

			while let Some((x, y, z)) = visiting_tracker.voxels.iter()
			.copied()
			.find(|a| !visited_tracker.voxels.contains(a))
			.map(|a| visiting_tracker.unaddress(a))
			{
				visiting_tracker.adjacent(x, y, z).into_iter()
					.flatten()
					.for_each(|(x, y, z)| {
						if grid.voxels.contains(&grid.address(x, y, z)) {
							faces += 1;
						} else {
							visiting_tracker.set(x, y, z);
						}
					});
				visited_tracker.set(x, y ,z);
			}

			println!("{}", faces);
		}
	}
}