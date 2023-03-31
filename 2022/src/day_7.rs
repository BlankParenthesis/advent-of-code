use std::collections::HashMap;

#[derive(Debug)]
struct Node<T> {
	data: T,
	children: Vec<usize>,
	parent: Option<usize>,
}

#[derive(Debug, Default)]
pub struct Tree<T> {
	nodes: Vec<Node<T>>
}

impl<T> Tree<T> {
	fn push_root(&mut self, data: T) -> usize {
		self.nodes.push(Node {
			data,
			parent: None,
			children: vec![],
		});
		self.nodes.len() - 1
	}

	fn node(&self, node_id: usize) -> Option<&Node<T>> {
		self.nodes.get(node_id)
	}

	fn node_mut(&mut self, node_id: usize) -> Option<&mut Node<T>> {
		self.nodes.get_mut(node_id)
	}

	fn children(&self, node_id: usize) -> Option<&Vec<usize>> {
		self.node(node_id).map(|n| &n.children)
	}

	fn push_child(&mut self, data: T, parent: usize) -> usize {
		self.nodes.push(Node {
			data,
			parent: Some(parent),
			children: vec![],
		});
		let id = self.nodes.len() - 1;

		self.nodes[parent].children.push(id);

		id
	}

	fn len(&self) -> usize {
		self.nodes.len()
	}
}

#[derive(Debug, Default)]
pub struct Directory {
	name: String,
	files: HashMap<String, usize>,
}

impl Directory {
	fn new(name: String) -> Self {
		Self { name, files: HashMap::default() }
	}
}

#[derive(Debug)]
pub enum CommandError {
	MissingCommand,
	MissingArgument,
	NoDirectory(String),
	BadList,
	BadSize(String),
}

impl Directory {
	pub fn create_from_str(input: &str) -> Result<Tree<Self>, CommandError> {
		let commands = input.split("$ ")
			.skip(1)
			.map(str::trim_end);

		let mut tree = Tree::<Directory>::default();
		let root = tree.push_root(Directory::new("/".to_owned()));
		let mut current_directory = root;
	
		for execution in commands {
			let mut lines = execution.split('\n');
			let command = lines.next().unwrap();

			let (command, arguments) = command.split_once(' ').unwrap_or((command, ""));

			match command {
				"cd" => {
					if arguments.is_empty() {
						return Err(CommandError::MissingArgument);
					}

					if arguments == "/" {
						current_directory = root;
					} else if arguments == ".." {
						current_directory = tree.node(current_directory).unwrap().parent.unwrap();
					} else {
						current_directory = tree.children(current_directory).unwrap()
							.iter().copied()
							.find(|c| tree.node(*c).unwrap().data.name == arguments)
							.ok_or_else(|| CommandError::NoDirectory(arguments.to_owned()))?;
					}
				},
				"ls" => {
					for entry in lines {
						let (prefix, name) = entry.split_once(' ').ok_or(CommandError::BadList)?;
						if prefix == "dir" {
							tree.push_child(Directory::new(name.to_owned()), current_directory);
						} else {
							let size = prefix.parse::<usize>()
								.map_err(|_| CommandError::BadSize(prefix.to_owned()))?;

							tree.node_mut(current_directory).unwrap()
								.data
								.files
								.insert(name.to_owned(), size);
						}
					}
				},
				_ => return Err(CommandError::MissingCommand),
			}
		}

		Ok(tree)
	}

	fn size(&self) -> usize {
		self.files.values().sum()
	}

	pub fn recursive_size(tree: &Tree<Self>, dir: usize) -> usize {
		let children_size: usize = tree.children(dir).unwrap()
			.iter()
			.map(|c| Directory::recursive_size(tree, *c))
			.sum();

		return children_size + tree.node(dir).unwrap().data.size()
	}
}

pub fn solve_a(tree: &Tree<Directory>) -> usize {
	const MAX_SIZE: usize = 100_000;

	(0..tree.len()).into_iter()
		.map(|i| Directory::recursive_size(tree, i))
		.filter(|s| *s <= MAX_SIZE)
		.sum()
}

pub fn solve_b(tree: &Tree<Directory>) -> usize {
	const TOTAL: usize = 70_000_000;
	const REQUIRED: usize = 30_000_000;

	let used = Directory::recursive_size(tree, 0);

	let free = TOTAL - used;
	let required_delete = REQUIRED - free;

	(0..tree.len()).into_iter()
		.map(|i| Directory::recursive_size(tree, i))
		.filter(|s| *s >= required_delete)
		.reduce(usize::min)
		.unwrap()
}