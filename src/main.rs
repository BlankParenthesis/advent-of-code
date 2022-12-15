use std::{path::PathBuf, str::Utf8Error, ops::RangeInclusive, collections::{HashSet, HashMap}, cmp::Ordering};

use clap::{Parser, builder::PossibleValue};
use nom::{bytes::complete::tag, sequence::{preceded, tuple, delimited}, combinator::{map_res, map}, character::complete::digit1, multi::separated_list0, Parser as NomParser};

mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_7;

pub(crate) trait Input: Sized {
	type Error;

	fn parse(data: &[u8]) -> Result<Self, Self::Error>
	where Self::Error: From<Utf8Error> {
		let string = std::str::from_utf8(data).map_err(Into::into)?;
		<Self as Input>::parse_str(string)
	}

	fn parse_str(data: &str) -> Result<Self, Self::Error>;
}

#[derive(Debug, Clone, clap::ValueEnum)]
enum Part {
	A,
	B,
}

#[derive(Debug, Clone)]
enum Day {
	One,
	Two,
	Three,
	Four,
	Five,
	Six,
	Seven,
	Eight,
	Nine,
	Ten,
	Eleven,
	Twelve,
	Thirteen,
	Fourteen,
	Fifteen,
}

impl clap::ValueEnum for Day {
	fn value_variants<'a>() -> &'a [Self] {
		&[
			Day::One,
			Day::Two,
			Day::Three,
			Day::Four,
			Day::Five,
			Day::Six,
			Day::Seven,
			Day::Eight,
			Day::Nine,
			Day::Ten,
			Day::Eleven,
			Day::Twelve,
			Day::Thirteen,
			Day::Fourteen,
			Day::Fifteen,
		]
	}

	fn to_possible_value(&self) -> Option<PossibleValue> {
		match self {
			Day::One => Some(PossibleValue::new("1").aliases(&["one", "1st", "first"])),
			Day::Two => Some(PossibleValue::new("2").aliases(&["two", "2nd", "second"])),
			Day::Three => Some(PossibleValue::new("3").aliases(&["three", "3rd", "third"])),
			Day::Four => Some(PossibleValue::new("4").aliases(&["four", "4th", "fourth"])),
			Day::Five => Some(PossibleValue::new("5").aliases(&["five", "5th", "fifth"])),
			Day::Six => Some(PossibleValue::new("6").aliases(&["six", "6th", "sixth"])),
			Day::Seven => Some(PossibleValue::new("7").aliases(&["seven", "7th", "seventh"])),
			Day::Eight => Some(PossibleValue::new("8").aliases(&["eight", "8th", "eighth"])),
			Day::Nine => Some(PossibleValue::new("9").aliases(&["nine", "9th", "ninth"])),
			Day::Ten => Some(PossibleValue::new("10").aliases(&["ten", "10th", "tenth"])),
			Day::Eleven => Some(PossibleValue::new("11").aliases(&["eleven", "11th", "eleventh"])),
			Day::Twelve => Some(PossibleValue::new("12").aliases(&["twelve", "12th", "twelfth"])),
			Day::Thirteen => Some(PossibleValue::new("13").aliases(&["thirteen", "13th", "thirteenth"])),
			Day::Fourteen => Some(PossibleValue::new("14").aliases(&["fourteen", "14th", "fourteenth"])),
			Day::Fifteen => Some(PossibleValue::new("15").aliases(&["fifteen", "15th", "fifteenth"])),
		}
	}
}

#[derive(Parser, Debug)]
struct Args {
	#[arg(value_enum)]
	day: Day,
	#[arg(value_enum)]
	part: Part,
	input_path: PathBuf,
}

fn take_positive_number<Num>(input: &str) -> nom::IResult<&str, Num>
where Num: std::str::FromStr {
	map_res(digit1, str::parse)(input)
}

fn take_negative_number<Num>(input: &str) -> nom::IResult<&str, Num>
where Num: std::str::FromStr {
	preceded(tag("-"), map_res(digit1, |d: &str| str::parse(("-".to_owned() + d).as_str())))(input)
}

fn take_number<Num>(input: &str) -> nom::IResult<&str, Num>
where Num: std::str::FromStr {
	take_positive_number(input)
	.or_else(|_| take_negative_number(input))
}

fn main() {
	let args = Args::parse();

	let data = std::fs::read(&args.input_path).expect("invalid path");

	match (args.day, &args.part) {
		(Day::One, Part::A) => {
			let input = day_1::CalorieList::parse(&data).expect("input parse error");
			println!("{}", input.top());
		},
		(Day::One, Part::B) => {
			let input = day_1::CalorieList::parse(&data).expect("input parse error");
			println!("{:?}", input.top_n(3));
		},
		(Day::Two, Part::A) => {
			let input = day_2::ActionStrategyGuide::parse(&data).expect("input parse error");
			println!("{}", input.score());
		},
		(Day::Two, Part::B) => {
			let input = day_2::OutcomeStrategyGuide::parse(&data).expect("input parse error");
			println!("{}", input.score());
		},
		(Day::Three, Part::A) => {
			let input = day_3::Packing::parse(&data).expect("input parse error");
			println!("{}", input.wrong_item_priority_sum().expect("solve error"));
		},
		(Day::Three, Part::B) => {
			let input = day_3::Packing::parse(&data).expect("input parse error");
			println!("{}", input.badges_priority_sum().expect("solve error"));
		},
		(Day::Four, _) => {
			fn parse_range(range: &str) -> RangeInclusive<usize> {
				let (start, end) = range.split_once('-').unwrap();
				let start = start.parse::<usize>().unwrap();
				let end = end.parse::<usize>().unwrap();
				start..=end
			}

			let pairs = std::str::from_utf8(&data).unwrap()
				.split('\n')
				.map(|pair| {
					let (a, b) = pair.split_once(',').unwrap();
					(parse_range(a), parse_range(b))
				})
				.collect::<Vec<(RangeInclusive<usize>, RangeInclusive<usize>)>>();
			

			match args.part {
				Part::A => {
					let overlapping = pairs.iter().filter(|(a, b)| {
						b.clone().step_by(1).all(|v| a.contains(&v)) ||
						a.clone().step_by(1).all(|v| b.contains(&v))
					});
		
					println!("{:?}", overlapping.count())
				},
				Part::B => {
					let overlapping = pairs.iter().filter(|(a, b)| {
						b.clone().step_by(1).any(|v| a.contains(&v))
					});
		
					println!("{:?}", overlapping.count())
				},
			}
		},
		(Day::Five, _) => {
			let (arrangement, instructions) = std::str::from_utf8(&data).unwrap()
				.split_once("\n\n").unwrap();

				struct Instruction {
				source: usize,
				destination: usize,
				count: usize,
			}

			let instructions = instructions.split('\n').map(|i| {
				let (_, (count, source, destination)) = tuple((
					preceded(tag("move "), take_positive_number),
					preceded(tag(" from "), take_positive_number),
					preceded(tag(" to "), take_positive_number),
				))(i).unwrap();

				Instruction { source, destination, count }
			});

			let mut arrangement = arrangement.split('\n').rev();
			let labels = arrangement.next().unwrap()
				.split_whitespace()
				.filter(|n| !n.is_empty())
				.map(|n| n.parse::<usize>().unwrap())
				.collect::<Vec<usize>>();

			let mut stacks = (0..labels.len())
				.map(|_| std::collections::VecDeque::<char>::new())
				.collect::<Vec<_>>();

			for line in arrangement {
				for stack in 0..labels.len() {
					let pos = stack * 4 + 1;
					let c = line.as_bytes()[pos] as char;
					if c != ' ' {
						stacks[stack].push_back(c)
					}
				}
			}


			for Instruction { source, destination: dest, count }  in instructions {
				let source = labels.iter().position(|i| *i == source).unwrap();
				let dest = labels.iter().position(|i| *i == dest).unwrap();

				match args.part {
					Part::A => {
						for _ in 0..count {
							let value = stacks[source].pop_back().unwrap();
							stacks[dest].push_back(value);
						}
					},
					Part::B => {
						let mut buffer = std::collections::VecDeque::new();
						for _ in 0..count {
							let value = stacks[source].pop_back().unwrap();
							buffer.push_front(value);
						}
						stacks[dest].append(&mut buffer);
					},
				}
			}

			
			let message = stacks.iter()
				.map(|stack| stack.back().unwrap_or(&' '))
				.collect::<String>();
			println!("{}", message);
		},
		(Day::Six, _) => {
			let size = match args.part {
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
		},
		(Day::Seven, _) => {
			let tree = day_7::Directory::create_from_str(std::str::from_utf8(&data).unwrap()).unwrap();

			match args.part {
				Part::A => {
					println!("{:?}", day_7::solve_a(&tree));
				},
				Part::B => {
					println!("{:?}", day_7::solve_b(&tree));
				},
			};
		},
		(Day::Eight, _) => {
			let grid = std::str::from_utf8(&data).unwrap()
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

			match args.part {
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
		},
		(Day::Nine, _) => {
			let input = std::str::from_utf8(&data).expect("input parse error");

			enum Direction {
				Right,
				Left,
				Down,
				Up,
			}

			struct Movement {
				direction: Direction,
				count: isize,
			}

			impl TryFrom<char> for Direction {
				type Error = ();

				fn try_from(c: char) -> Result<Self, Self::Error> {
					match c {
						'R' => Ok(Self::Right),
						'L' => Ok(Self::Left),
						'D' => Ok(Self::Down),
						'U' => Ok(Self::Up),
						_ => Err(()),
					}
				}
			}

			let instructions = input.split('\n')
				.map(|line| {
					let (d, c) = line.split_once(' ').unwrap();
					let direction = Direction::try_from(d.chars().next().unwrap()).unwrap();
					let count = c.parse::<isize>().unwrap();

					Movement { direction, count }
				})
				.collect::<Vec<_>>();

			let rope_length = match args.part {
				Part::A => 2,
				Part::B => 10,
			};

			let mut visited_positions: HashSet<(isize, isize)> = HashSet::new();
			let mut positions = vec![(0, 0); rope_length];
			
			visited_positions.insert(*positions.last().unwrap());

			for instruction in instructions {
				for _ in 0..instruction.count {
					match instruction.direction {
						Direction::Down => positions[0].1 += 1,
						Direction::Left => positions[0].0 -= 1,
						Direction::Right => positions[0].0 += 1,
						Direction::Up => positions[0].1 -= 1,
					}

					for i in 1..positions.len() {
						let previous = positions[i - 1];
						let current = &mut positions[i];
						
						let diff_x = isize::abs_diff(previous.0, current.0);
						let diff_y = isize::abs_diff(previous.1, current.1);

						if diff_x > 1 || diff_y > 1 {
							current.0 += (previous.0 - current.0).clamp(-1, 1);
							current.1 += (previous.1 - current.1).clamp(-1, 1);
						}
					}

					visited_positions.insert(*positions.last().unwrap());
				}
			}

			println!("{}", visited_positions.len());
		},
		(Day::Ten, _) => {
			enum Instruction {
				NoOp,
				AddX(isize),
			}

			impl Instruction {
				fn execution_time(&self) -> usize {
					match self {
						Self::NoOp => 1,
						Self::AddX(_) => 2,
					}
				}
			}

			impl TryFrom<&str> for Instruction {
				type Error = ();

				fn try_from(string: &str) -> Result<Self, Self::Error> {
					if string == "noop" {
						Ok(Instruction::NoOp)
					} else {
						let (i, value) = string.split_once(' ').ok_or(())?;
						let value = value.parse::<isize>().map_err(|_| ())?;

						if i == "addx" {
							Ok(Instruction::AddX(value))
						} else {
							Err(())
						}
					}
				}
			}

			let instructions = std::str::from_utf8(&data).unwrap()
				.split('\n')
				.map(Instruction::try_from)
				.collect::<Result<Vec<_>, _>>()
				.unwrap();


			let mut x_reg = 1;
			let mut strength = 0_isize;
			
			const FIRST_CYCLE: isize = 20;
			const CYCLE_INTERVAL: isize = 40;

			let mut cycle = 0;

			for instruction in instructions {
				for _ in 0..instruction.execution_time() {
					cycle += 1;

					match args.part {
						Part::A => {
							if (cycle - FIRST_CYCLE) % CYCLE_INTERVAL == 0 {
								strength += cycle * x_reg;
							}
						},
						Part::B => {
							let position = (cycle - 1) % CYCLE_INTERVAL;

							if isize::abs_diff(x_reg, position) < 2 {
								print!("#");
							} else {
								print!(".");
							}
							
							if cycle % CYCLE_INTERVAL == 0 {
								println!();
							}
						},
					}
				}

				match instruction {
					Instruction::NoOp => (),
					Instruction::AddX(value) => x_reg += value,
				}
			}

			if matches![args.part, Part::A] {
				println!("{}", strength);
			}
		},
		(Day::Eleven, _) => {
			let monkeys = std::str::from_utf8(&data).unwrap()
				.split("\n\n");

			fn take_numbers<Num>(input: &str) -> nom::IResult<&str, Vec<Num>>
			where Num: std::str::FromStr{
				separated_list0(tag(", "), map_res(digit1, str::parse))(input)
			}

			#[derive(Debug, Clone)]
			enum Operation {
				Add(isize),
				Multiply(isize),
				Squared,
			}

			impl Operation {
				fn parse(input: &str) -> nom::IResult<&str, Self> {
					preceded(tag("new = old "), 
						map(preceded(tag("* "), take_positive_number), Operation::Multiply)
						.or(map(preceded(tag("+ "), take_positive_number), Operation::Add))
						.or(map(tag("+ old"), |_| Operation::Multiply(2)))
						.or(map(tag("* old"), |_| Operation::Squared))
					)(input)
				}

				fn perform(&self, input: isize) -> isize {
					match self {
						Self::Squared => input * input,
						Self::Add(value) => input + value,
						Self::Multiply(value) => input * value,
					}
				}
			}

			#[derive(Debug, Clone)]
			struct Test {
				number: isize,
				value_true: usize,
				value_false: usize,
			}

			#[derive(Debug)]
			struct Monkey {
				id: isize,
				items: Vec<isize>,
				operation: Operation,
				test: Test,
			}

			let mut monkeys = monkeys.map(|i| {
				let (i, (_, id, _)) =          tuple((tag("Monkey "), take_positive_number, tag(":\n")))(i).unwrap();
				let (i, (_, items, _)) =       tuple((tag("  Starting items: "), take_numbers, tag("\n")))(i).unwrap();
				let (i, (_, operation, _)) =   tuple((tag("  Operation: "), Operation::parse, tag("\n")))(i).unwrap();
				let (i, (_, test_number, _)) = tuple((tag("  Test: divisible by "), take_positive_number, tag("\n")))(i).unwrap();
				let (i, (_, test_true, _)) =   tuple((tag("    If true: throw to monkey "), take_positive_number, tag("\n")))(i).unwrap();
				let (_, (_, test_false)) =     tuple((tag("    If false: throw to monkey "), take_positive_number))(i).unwrap();

				let test = Test {
					number: test_number,
					value_true: test_true,
					value_false: test_false,
				};

				Monkey { id, items, operation, test }
			})
			.collect::<Vec<_>>();

			monkeys.sort_by(|a, b| a.id.cmp(&b.id));

			let MAX_ROUNDS = match args.part {
				Part::A => 20,
				Part::B => 10000,
			};

			let WORRY_DECAY = match args.part {
				Part::A => 3,
				Part::B => 1,
			};

			let mut inspections = vec![0; monkeys.len()];

			// very simple gcd
			let gcd: isize = monkeys.iter()
				.map(|m| m.test.number)
				.product();

			for round in 1..=MAX_ROUNDS {
				for monkey_id in 0..monkeys.len() {
					let monkey = monkeys.get_mut(monkey_id).unwrap();

					let items = monkey.items.clone();
					inspections[monkey_id] += items.len();
					monkey.items = vec![];
					let op = monkey.operation.clone();
					let test = monkey.test.clone();


					for old in items {
						let new = (op.perform(old) / WORRY_DECAY) % gcd;

						let pass_monkey = if new % test.number == 0 {
							test.value_true
						} else {
							test.value_false
						};

						monkeys.get_mut(pass_monkey).unwrap().items.push(new);
					}
				}
			}

			inspections.sort();
			let monkey_business: usize = inspections.iter().rev().take(2).product();

			println!("{}", monkey_business);
		},
		(Day::Twelve, _) => {

			enum Node {
				Start,
				End,
				Walkable(i8),
			}

			impl Node {
				fn height(&self) -> i8 {
					match self {
						Node::End => 'z' as i8,
						Node::Start => 'a' as i8,
						Node::Walkable(v) => *v,
					}
				}
			}
			
			let map = std::str::from_utf8(&data).unwrap()
				.split('\n')
				.map(|line| {
					line.chars().map(|c| {
						match c {
							'S' => Node::Start,
							'E' => Node::End,
							c => Node::Walkable(c as i8),
						}
					})
					.collect()
				})
				.collect::<Vec<Vec<Node>>>();

			let height = map.len();
			let width = map[0].len();
			
			let mut graph = petgraph::Graph::<(), ()>::new();

			const OFFSETS: &[(isize, isize); 4] = &[
				(1, 0),
				(-1, 0),
				(0, 1),
				(0, -1),
			];

			let ids = (0..width).into_iter().map(|x| {
				(0..width).into_iter().map(|y| {
					graph.add_node(())
				}).collect()
			}).collect::<Vec<Vec<petgraph::graph::NodeIndex>>>();

			for x in 0..width {
				for y in 0..height {
					let node = &map[y][x];
					let height = node.height();
					let gid = ids[y][x];

					for (ax, ay) in OFFSETS.iter().map(|(ox, oy)| (x as isize + ox, y as isize + oy)) {
						if ax >= 0 && ay >= 0 {
							let other = map.get(ay as usize).and_then(|v| v.get(ax as usize));
							if let Some(other) = other {
								if height - other.height() >= -1 {
									let gido = ids[ay as usize][ax as usize];
									graph.extend_with_edges(&[
										(gid, gido),
									])
								}
							}
						}

					}
				}
			}
			
			let start = map.iter().flatten().position(|m| matches!(m, Node::Start)).unwrap();
			let start = ids[start / width][start % width];

			let end = map.iter().flatten().position(|m| matches!(m, Node::End)).unwrap();
			let end = ids[end / width][end % width];

			match args.part {
				Part::A => {
					let path = petgraph::algo::dijkstra(&graph, start, Some(end), |_| 1);
					println!("{:?}", path.get(&end));
				},	
				Part::B => {
					graph.reverse();
					let path = petgraph::algo::dijkstra(&graph, end, None, |_| 1);
					let lowest = map.iter()
						.flatten()
						.enumerate()
						.filter(|(_, m)| m.height() == b'a' as i8)
						.filter_map(|(id, _)| path.get(&ids[id / width][id % width]))
						.copied()
						.reduce(u32::min);

					println!("{:?}", lowest);
				},
			}
		},
		(Day::Thirteen, _) => {
			#[derive(Debug, Clone, PartialEq, Eq)]
			enum Value {
				Int(u8),
				List(Vec<Value>),
			}

			impl Value {
				fn parse(input: &str) -> nom::IResult<&str, Self> {
					let as_int = map(take_positive_number, Self::Int)(input);
					let as_list = map(delimited(tag("["), separated_list0(tag(","), Value::parse), tag("]")), Self::List)(input);
					as_int.or(as_list)
				}
			}

			impl PartialOrd for Value {
				fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
					match (self, other) {
						(Value::Int(a), Value::Int(b)) => a.partial_cmp(b),
						(Value::Int(a), Value::List(_)) => {
							Value::List(vec![Value::Int(*a)]).partial_cmp(other)
						},
						(Value::List(_), Value::Int(b)) => {
							self.partial_cmp(&Value::List(vec![Value::Int(*b)]))
						},
						(Value::List(a), Value::List(b)) => {
							a.partial_cmp(b)
						},
					}
				}
			}
		
			impl Ord for Value {
				fn cmp(&self, other: &Self) -> Ordering {
					self.partial_cmp(other).unwrap()
				}
			}

			match args.part {
				Part::A => {
					let pairs = std::str::from_utf8(&data).unwrap()
						.split("\n\n")
						.map(|p| {
							let (a, b) = p.split_once('\n').unwrap();
							(Value::parse(a).unwrap().1, Value::parse(b).unwrap().1)
						})
						.collect::<Vec<_>>();

					let sorted = pairs.iter()
						.map(|(a, b)| matches!(a.cmp(b), Ordering::Less))
						.enumerate()
						.filter_map(|(i, o)| if o { Some(i + 1) } else { None })
						.sum::<usize>();

					println!("{:?}", sorted);
				},
				Part::B => {
					let markers = [
						Value::List(vec![Value::List(vec![Value::Int(2)])]),
						Value::List(vec![Value::List(vec![Value::Int(6)])]),
					];

					let mut packets = std::str::from_utf8(&data).unwrap()
						.split('\n')
						.filter(|s| !s.is_empty())
						.map(|p| Value::parse(p).unwrap().1)
						.chain(markers.clone())
						.collect::<Vec<_>>();

					packets.sort();

					let decoder_key = markers.iter()
						.map(|m| packets.iter().position(|v| v.eq(m)).unwrap())
						.map(|p| p + 1)
						.product::<usize>();
					
					println!("{}", decoder_key);
				},
			}
		},
		(Day::Fourteen, _) => {			
			fn parse_path<Num>(input: &str) -> nom::IResult<&str, Vec<(Num, Num)>>
			where Num: std::str::FromStr {
				separated_list0(tag(" -> "), tuple((take_positive_number, tag(","), take_positive_number)))(input)
					.map(|(s, vec)| {
						(s, vec.into_iter()
							.map(|(x, _, y)| (x, y))
							.collect())
					})
			}

			let paths = std::str::from_utf8(&data).unwrap().split('\n')
				.map(parse_path)
				.map(Result::unwrap)
				.map(|(_, path)| path)
				.collect::<Vec<Vec<(usize, usize)>>>();

			const SAND_INPOINT: (usize, usize) = (500, 0);

			let mut min_x = *paths.iter().flatten().chain([&SAND_INPOINT]).map(|(x, _)| x).min().unwrap();
			let min_y = *paths.iter().flatten().chain([&SAND_INPOINT]).map(|(_, y)| y).min().unwrap();
			let mut max_x = *paths.iter().flatten().chain([&SAND_INPOINT]).map(|(x, _)| x).max().unwrap();
			let max_y = *paths.iter().flatten().chain([&SAND_INPOINT]).map(|(_, y)| y).max().unwrap();

			let extra_height = match args.part {
				Part::A => 0,
				Part::B => 1,
			};
			
			let height = max_y - min_y + 1 + extra_height;

			if matches!(args.part, Part::B) {
				min_x = SAND_INPOINT.0 - height;
				max_x = SAND_INPOINT.0 + height;
			}

			let width = max_x - min_x + 1;

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

			fn draw_grid(grid: &Vec<Vec<Tile>>, width: usize, height: usize) {
				for y in 0..height {
					for x in 0..width {
						print!("{}", grid[y][x]);
					}
					println!();
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
								if matches!(args.part, Part::A) {
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
						match args.part {
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

			match args.part {
				Part::A => println!("{}", sands - 1),
				Part::B => println!("{}", sands),
			}		
		},
		(Day::Fifteen, _) => {
			#[derive(Debug)]
			struct Sensor {
				pos: (isize, isize),
				beacon: (isize, isize),
				range: usize,
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

			let sensors = std::str::from_utf8(&data).unwrap()
				.split('\n')
				.map(|b| Sensor::parse(b).unwrap().1)
				.collect::<Vec<Sensor>>();

			let is_example = args.input_path.file_name().unwrap()
				.to_str().unwrap()
				.contains("example");

			match args.part {
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
	}
}
