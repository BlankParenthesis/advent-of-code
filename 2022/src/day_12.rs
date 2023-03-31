use crate::Part;

pub(crate) fn solve(data: &[u8], part: Part) {

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
	
	let map = std::str::from_utf8(data).unwrap()
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

	match part {
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
}