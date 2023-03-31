#[derive(Debug)]
pub struct BingoBoard {
	cells: Vec<Vec<usize>>,
	marks: Vec<(usize, usize)>,
}

impl BingoBoard {
	fn size(&self) -> (usize, usize) {
		(
			self.cells.len(),
			self.cells
				.first()
				.map(Vec::len)
				.unwrap_or(0),
		)
	}

	fn mark(&mut self, value: usize) {
		let marked = self.cells
			.iter()
			.enumerate()
			.map(|(y, row)| {
				row
					.iter()
					.enumerate()
					.filter(|(_, cell_value)| **cell_value == value)
					.map(move |(x, _)| (x, y))
			})
			.flatten()
			.collect::<Vec<(usize, usize)>>();

		for cell in marked {
			self.marks.push(cell);
		}
	}

	fn has_bingo(&self) -> bool {
		let (width, height) = self.size();
		
		let row_win = (0..height).any(|y| {
			(0..width).all(|x| self.marks.contains(&(x, y)))
		});
		
		let column_win = (0..width).any(|y| {
			(0..height).all(|x| self.marks.contains(&(x, y)))
		});
		
		row_win || column_win
	}

	fn marked_tiles<'l>(&'l self) -> impl Iterator<Item = usize> + 'l {
		self.marks.iter().map(|(x, y)| self.cells[*y][*x])
	}

	fn unmarked_tiles<'l>(&'l self) -> impl Iterator<Item = usize> + 'l {
		self.cells.iter().enumerate().map(move |(y, row)| {
			row
				.iter()
				.enumerate()
				.filter(move |(x, _)| !self.marks.contains(&(*x, y)))
				.map(|(_, cell)| *cell)
		})
		.flatten()
	}

	fn score(&self, last_draw: usize) -> usize {
		self.unmarked_tiles().sum::<usize>() * last_draw
	}
}

pub struct BingoGame {
	number_source: Box<dyn Iterator<Item= usize>>,
	boards: Vec<BingoBoard>,
}

impl BingoGame {
	fn draw_next(&mut self) -> Option<usize> {
		let next = self.number_source.next();

		if let Some(next) = next {
			for board in self.boards.iter_mut() {
				board.mark(next);
			}
		}

		next
	}

	fn winners(&self) -> Vec<&BingoBoard> {
		self.boards.iter().filter(|board| board.has_bingo()).collect()
	}
}

pub fn parse_input(input: &str) -> Result<BingoGame, ()> {
	let (number_source, boards) = input.split_once("\n\n").ok_or(())?;
	let number_source = number_source
		.split(',')
		.map(|s| s.parse::<usize>())
		.collect::<Result<Vec<usize>, _>>()
		.map_err(|_| ())?;
	let number_source = Box::new(number_source.into_iter());
	let boards = boards.split("\n\n").into_iter().map(|board| {
		let cells = board.split('\n').map(|row| {
			row
				.split_ascii_whitespace()
				.map(|s| s.parse::<usize>())
				.collect::<Result<Vec<usize>, _>>()
				.map_err(|_| ())
		})
		.collect::<Result<_, _>>()?;

		Ok(BingoBoard {
			cells,
			marks: Vec::default(),
		})
	})
	.collect::<Result<Vec<_>, _>>()?;

	Ok(BingoGame {
		number_source,
		boards,
	})
}

pub fn solve(mut input: BingoGame) -> usize {
	while let Some(last_draw) = input.draw_next() {
		let mut winners = input.winners();
		if !winners.is_empty() {
			winners.sort_by(|a, b| a.score(last_draw).cmp(&b.score(last_draw)));
			return winners.first().unwrap().score(last_draw);
		}
	}

	println!("{:?}", input.boards);

	panic!("No winning boards");
}