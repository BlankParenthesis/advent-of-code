use std::collections::HashMap;

use nom::{combinator::map, sequence::{preceded, tuple}, bytes::complete::tag, character::complete::alpha1, multi::separated_list0, Parser};
use petgraph::prelude::NodeIndex;

use crate::{Part, take_positive_number};

#[derive(Debug)]
struct Valve {
	id: String,
	flow: usize,
	tunnels: Vec<String>,
}

impl Valve {
	fn parse(input: &str) -> nom::IResult<&str, Self> {
		map(tuple((
			preceded(tag("Valve "), alpha1),
			preceded(tag(" has flow rate="), take_positive_number),
			preceded(tag("; tunnel leads to valve ").or(tag("; tunnels lead to valves ")), separated_list0(tag(", "), alpha1)),
		)), |(id, flow, tunnels)| Self {
			id: id.to_owned(),
			flow,
			tunnels: tunnels.into_iter().map(|s| s.to_owned()).collect(),
		})(input)
	}
}

struct IndexedValve {
	flow: usize,
	tunnels: Vec<usize>,
}

pub(crate) fn solve(data: &[u8], part: Part) {

	let valves = std::str::from_utf8(data).unwrap()
		.split('\n')
		.map(|s| Valve::parse(s).unwrap().1)
		.collect::<Vec<_>>();

	let starting_location = valves.iter().position(|v| v.id == "AA").unwrap();

	let valves = valves.iter().map(|v| {
		let tunnels = v.tunnels.iter()
			.map(|t| valves.iter().position(|v| &v.id == t).unwrap())
			.collect::<Vec<usize>>();

		IndexedValve { flow: v.flow, tunnels }
	}).collect::<Vec<_>>();

	let time_budget = match part {
		Part::A => 30,
		Part::B => 26,
	};


	let mut graph = petgraph::Graph::<usize, _>::new();

	let indexes = valves.iter()
		.map(|v| graph.add_node(v.flow))
		.collect::<Vec<_>>();

	for (i, index) in indexes.iter().enumerate() {
		for edge in valves[i].tunnels.iter() {
			graph.add_edge(*index, indexes[*edge], ());
		}
	}

	let cost_table = indexes.iter().map(|i| {
		(*i, petgraph::algo::k_shortest_path(&graph, *i, None, 1, |_| 1))
	}).collect::<HashMap<_, _>>();

	let valuable_valves = indexes.iter()
		.filter(|i| *graph.node_weight(**i).unwrap() > 0)
		.copied()
		.collect::<Vec<_>>();

	fn best_flow(
		graph: &petgraph::Graph::<usize, ()>,
		cost_table: &HashMap<NodeIndex, HashMap<NodeIndex, usize>>,
		location: NodeIndex,
		valuable_valves: Vec<NodeIndex>,
		time_remaining: usize,
	) -> usize {
		let value = graph.node_weight(location).unwrap() * time_remaining;
		let valuable_valves = valuable_valves.into_iter()
			.filter(|v| *v != location)
			.collect::<Vec<_>>();

		let costs = cost_table.get(&location).unwrap();
		let best_subtree_value = valuable_valves.iter()
			.filter_map(|v| {
				let cost = *costs.get(v).unwrap();
				if cost < (time_remaining - 1) {
					Some((v, cost))
				} else {
					None
				}
			})
			.map(|(v, cost)| {
				best_flow(
					graph,
					cost_table,
					*v,
					valuable_valves.clone(),
					time_remaining - cost - 1,
				)
			})
			.max().unwrap_or(0);

		value + best_subtree_value
	}

	fn best_flow_pair(
		graph: &petgraph::Graph::<usize, ()>,
		cost_table: &HashMap<NodeIndex, HashMap<NodeIndex, usize>>,
		location: (NodeIndex, NodeIndex),
		valuable_valves: Vec<NodeIndex>,
		time_remaining: (usize, usize),
	) -> usize {
		// if b is ahead, we should work on a, otherwise b
		let b_ahead = time_remaining.0 > time_remaining.1;

		let used_location = if b_ahead { location.0 } else { location.1 };
		let used_time_remaining = if b_ahead { time_remaining.0 } else { time_remaining.1 };

		let costs = cost_table.get(&used_location).unwrap();

		valuable_valves.iter()
			.filter_map(|valve| {
				// cost + 1 to account for then turning the valve
				let cost = *costs.get(valve).unwrap() + 1;
				if cost < used_time_remaining {
					Some((valve, cost))
				} else {
					None
				}
			})
			.map(|(valve, cost)| {
				let time = used_time_remaining - cost;
				let value = graph.node_weight(*valve).unwrap() * time;

				let valuable_valves = valuable_valves.iter()
					.filter(|v| *v != valve)
					.cloned()
					.collect::<Vec<_>>();


				value + best_flow_pair(
					graph,
					cost_table,
					if b_ahead {
						(*valve, location.1)
					} else {
						(location.0, *valve)
					},
					valuable_valves,
					if b_ahead {
						(time, time_remaining.1)
					} else {
						(time_remaining.0, time)
					},
				)
			})
			.max().unwrap_or(0)
	}

	match part {
		Part::A => {
			let best_pressure = best_flow(
				&graph,
				&cost_table,
				indexes[starting_location],
				valuable_valves,
				time_budget,
			);

			println!("{}", best_pressure);
		},
		Part::B => {
			let best_pressure = best_flow_pair(
				&graph,
				&cost_table,
				(indexes[starting_location], indexes[starting_location]),
				valuable_valves,
				(time_budget, time_budget),
			);

			println!("{}", best_pressure);
		},
	}
}