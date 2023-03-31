use crate::Part;

pub(crate) fn solve(data: &[u8], part: Part) {
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

			match part {
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

	if matches![part, Part::A] {
		println!("{}", strength);
	}
}