use nom::{sequence::{tuple, preceded}, bytes::complete::tag, combinator::{map, map_res}, Parser, multi::separated_list0, character::complete::digit1};

use crate::{Part, take_positive_number};

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

pub(crate) fn solve(data: &[u8], part: Part) {
	let monkeys = std::str::from_utf8(data).unwrap()
		.split("\n\n");

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

	let MAX_ROUNDS = match part {
		Part::A => 20,
		Part::B => 10000,
	};

	let WORRY_DECAY = match part {
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
}