use std::convert::TryFrom;

pub enum Command {
	Forward(usize),
	Down(usize),
	Up(usize),
}

impl TryFrom<&str> for Command {
	type Error = ();

	fn try_from(string: &str) -> Result<Self, Self::Error> {
		let (command, value) = string
			.split_once(' ')
			.ok_or(())?;

		let value = value.parse::<usize>().map_err(|_| ())?;

		match command.to_lowercase().as_ref() {
			"forward" => Ok(Command::Forward(value)),
			"down" => Ok(Command::Down(value)),
			"up" => Ok(Command::Up(value)),
			_ => Err(()),
		}
	}
}

pub fn parse_input(input: &str) -> Result<Vec<Command>, ()> {
	input.split('\n').map(Command::try_from).collect()
}

pub fn solve(input: Vec<Command>) -> usize {
	let horizontal_position = input
		.iter()
		.fold(0, |sum, command| {
			match command {
				Command::Forward(value) => usize::checked_add(sum, *value).unwrap(),
				Command::Down(_) => sum,
				Command::Up(_) => sum,
			}
		});

	let depth = input
		.iter()
		.fold(0, |sum, command| {
			match command {
				Command::Forward(_) => sum,
				Command::Down(value) => usize::checked_add(sum, *value).unwrap(),
				Command::Up(value) => usize::checked_sub(sum, *value).expect("Submarines don't fly"),
			}
		});
	
	depth * horizontal_position
}