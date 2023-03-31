use nom::{combinator::map, sequence::{tuple, preceded}, bytes::complete::tag};

use crate::{Part, take_number};

#[derive(Debug)]
struct Sensor {
	pos: (isize, isize),
	beacon: (isize, isize),
	range: usize,
}

impl Sensor {
	fn parse(input: &str) -> nom::IResult<&str, Self> {
		map(tuple((
			preceded(tag("Sensor at "), take_coord),
			preceded(tag(": closest beacon is at "), take_coord),
		)), |(pos, nearest)| {
			Self { pos, beacon: nearest, range: man_distance(pos, nearest) }
		})(input)
	}

	fn covers(&self, point: (isize, isize)) -> bool {
		self.range >= man_distance(self.pos, point)
	}
}

fn take_coord<Num>(input: &str) -> nom::IResult<&str, (Num, Num)>
where Num: std::str::FromStr {
	tuple((
		preceded(tag("x="), take_number),
		preceded(tag(", y="), take_number),
	))(input)
}

fn man_distance(a: (isize, isize), b: (isize, isize)) -> usize {
	isize::abs_diff(a.0, b.0) + isize::abs_diff(a.1, b.1)
}

pub(crate) fn solve(data: &[u8], part: Part, is_example: bool) {

	let sensors = std::str::from_utf8(data).unwrap()
		.split('\n')
		.map(|b| Sensor::parse(b).unwrap().1)
		.collect::<Vec<Sensor>>();

	match part {
		Part::A => {
			let min_x = sensors.iter().map(|s| s.pos.0 - s.range as isize).min().unwrap();
			let max_x = sensors.iter().map(|s| s.pos.0 + s.range as isize).max().unwrap();

			let y = if is_example {10} else {2000000};
			let sensors = sensors.iter().filter(|s| s.range >= isize::abs_diff(s.pos.1, y));
			
			let covered = (min_x..=max_x).into_iter().filter(|x| {
				let is_covered = sensors.clone().any(|s| s.range >= man_distance(s.pos, (*x, y)));
				let is_beacon = sensors.clone().any(|s| s.beacon.0 == *x && s.beacon.1 == y);
				//match (is_covered, is_beacon) {
				//	(_, true) => print!("B"),
				//	(true, false) => print!("#"),
				//	(false, false) => print!("."),
				//}
				!is_beacon && is_covered
			}).count();

			//println!();
			println!("{}", covered);
		},
		Part::B => {
			fn all_covered(sensors: &[Sensor], point: &[(isize, isize)]) -> bool {
				sensors.iter().any(|s| {
					point.iter().all(|p| s.covers(*p))
				})
			}
			
			fn search_area(
				sensors: &[Sensor],
				start: (isize, isize),
				end: (isize, isize),
			) -> Option<(isize, isize)> {
				//println!("searching: x({}–{}), y({}–{})", start.0, end.0, start.1, end.1);

				if start.0 > end.0 || start.1 > end.1 {
					return None;
				}

				if start == end {
					if sensors.iter().any(|s| s.covers(start)) {
						None
					} else {
						Some(start)
					}
				} else {
					let covered = all_covered(sensors, &[start, end, (start.0, end.1), (end.0, start.1)]);
					
					if covered {
						None
					} else {
						let mid = ((start.0 + end.0) / 2, (start.1 + end.1) / 2);

						search_area(sensors, start, mid)
						.or_else(|| search_area(sensors, (mid.0 + 1, start.1), (end.0, mid.1 + 1)))
						.or_else(|| search_area(sensors, (start.0, mid.1 + 1), (mid.0 + 1, end.1)))
						.or_else(|| search_area(sensors, (mid.0 + 1, mid.1 + 1), end))
					}
				}
			}


			let min = 0;
			let max = if is_example {20} else {4000000};

			//for y in min..=max {
			//	for x in min..=max {
			//		let is_sensor = sensors.iter().any(|s| s.pos == (x, y));
			//		let is_covered = sensors.iter().any(|s| s.covers((x, y)));
			//		let is_beacon = sensors.iter().any(|s| s.beacon == (x, y));
			//		if is_sensor {
			//			print!("S");
			//		} else if is_beacon {
			//			print!("B");
			//		} else if is_covered {
			//			print!("#");
			//		} else {
			//			print!(".");
			//		}
			//	}
			//	println!();
			//}

			let tuning_freq = search_area(&sensors, (min, min), (max, max));

			println!("{}, {:?} {:?}", sensors.iter().any(|s| s.covers(tuning_freq.unwrap())), tuning_freq, tuning_freq.map(|(x, y)| x * 4000000 + y));
		},
	};
}