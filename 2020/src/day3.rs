extern crate ndarray;

use crate::util::*;
use std::error::Error;
use ndarray::Array2;

#[derive(Debug, Clone)]
struct UnknownCharacter {
	character: char,
}
impl Error for UnknownCharacter {}
impl std::fmt::Display for UnknownCharacter {
	fn fmt(&self, formatter: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error>{
		write!(formatter, "encountered an unknown character: {}", self.character)
	}
}

#[derive(PartialEq)]
enum MapKey {
	Open,
	Tree,
}

impl MapKey {
	fn parse(c: &char) -> Result<MapKey, Box<dyn Error>> {
		match c {
			'.' => Ok(MapKey::Open),
			'#' => Ok(MapKey::Tree),
			_ => Err(UnknownCharacter { character: *c }.into()),
		}
	}
}

type Position = (usize, usize);
type Path = Vec<Position>;

struct Map {
	width: usize,
	height: usize,
	data: Array2<MapKey>,
}

impl Map {
	fn parse(map: Vec<Vec<char>>) -> Result<Map, Box<dyn Error>> {
		let height = map.len();
		let width = map.iter().map(|row| row.len()).min().unwrap_or(0);

		let raw = map.iter()
			.flat_map(|v| v.iter().map(|c| MapKey::parse(c)))
			.collect::<Result<Vec<MapKey>, Box<dyn Error>>>()?;

		let data: Array2<MapKey> = Array2::from_shape_vec((width, height), raw)?;

		Ok(Map {
			width,
			height,
			data,
		})
	}

	fn get(&self, pos: Position) -> &MapKey {
		let x = pos.0 % self.width;
		let y = pos.1 % self.height;
		&self.data[[y, x]]
	}
}

impl std::fmt::Display for Map {
	fn fmt(&self, formatter: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error>{
		for y in 0..self.height {
			let row: String = (0..self.width).map(|x| match self.get((x, y)){
				MapKey::Open => ".",
				MapKey::Tree => "#",
			}).collect();
			
			formatter.write_str(row.as_str())?;
			formatter.write_str("\n")?;
		}
		Ok(())
	}
}

fn solve() -> usize {
	let map = Map::parse(read_file("resources/day3/map.txt", |s| s.chars().collect())).unwrap();
	(0..map.height).map(|y| map.get((y * 3, y))).filter(|k| **k == MapKey::Tree).count()
}

pub fn print_solution() {
	print_day(3);
	let tree_count = solve();
	println!("{} {} encountered", tree_count, "tree".to_owned().pluralize(tree_count));
}