use std::cmp::Ordering;

use nom::{sequence::delimited, combinator::map, bytes::complete::tag, multi::separated_list0};

use crate::{Part, take_positive_number};

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

pub(crate) fn solve(data: &[u8], part: Part) {
	match part {
		Part::A => {
			let pairs = std::str::from_utf8(data).unwrap()
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

			let mut packets = std::str::from_utf8(data).unwrap()
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
}