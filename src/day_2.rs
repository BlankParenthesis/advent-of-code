use std::str::Utf8Error;

use crate::Input;

#[derive(Debug)]
pub(crate) enum StrategyGuideParseError {
	Utf8Error(Utf8Error),
	UnknownAction,
	MalformedRound,
	UnknownOutcome,
}

impl From<Utf8Error> for StrategyGuideParseError {
    fn from(err: Utf8Error) -> Self {
        Self::Utf8Error(err)
    }
}

#[derive(Debug, Clone, Copy)]
enum Action {
	Rock,
	Paper,
	Scissors,
}

impl Action {
	fn score(&self) -> usize {
		match self {
			Action::Rock => 1,
			Action::Paper => 2,
			Action::Scissors => 3,
		}
	}
}


impl Action {
	fn try_from_opponent(s: char) -> Result<Self, StrategyGuideParseError> {
		match s {
			'A' => Ok(Action::Rock),
			'B' => Ok(Action::Paper),
			'C' => Ok(Action::Scissors),
			_ => Err(StrategyGuideParseError::UnknownAction)
		}
	}

	fn try_from_you(s: char) -> Result<Self, StrategyGuideParseError> {
		match s {
			'X' => Ok(Action::Rock),
			'Y' => Ok(Action::Paper),
			'Z' => Ok(Action::Scissors),
			_ => Err(StrategyGuideParseError::UnknownAction)
		}
	}
}

enum RoundOutcome {
	Win,
	Draw,
	Lose,
}

impl RoundOutcome {
	fn score(&self) -> usize {
		match self {
			RoundOutcome::Win => 6,
			RoundOutcome::Draw => 3,
			RoundOutcome::Lose => 0,
		}
	}
}

impl RoundOutcome {
	fn try_from_char(s: char) -> Result<Self, StrategyGuideParseError> {
		match s {
			'X' => Ok(RoundOutcome::Lose),
			'Y' => Ok(RoundOutcome::Draw),
			'Z' => Ok(RoundOutcome::Win),
			_ => Err(StrategyGuideParseError::UnknownOutcome)
		}
	}

}

#[derive(Debug)]
pub(crate) struct Round {
	opponent: Action,
	you: Action,
}

impl Round {
    fn try_from_action(value: &str) -> Result<Self, StrategyGuideParseError> {
		let opponent = value.chars().nth(0)
			.ok_or(StrategyGuideParseError::MalformedRound)
			.and_then(Action::try_from_opponent)?;
		let you = value.chars().nth(2)
			.ok_or(StrategyGuideParseError::MalformedRound)
			.and_then(Action::try_from_you)?;

		Ok(Self { opponent, you })
    }

    fn try_from_outcome(value: &str) -> Result<Self, StrategyGuideParseError> {
		let opponent = value.chars().nth(0)
			.ok_or(StrategyGuideParseError::MalformedRound)
			.and_then(Action::try_from_opponent)?;
		let outcome = value.chars().nth(2)
			.ok_or(StrategyGuideParseError::MalformedRound)
			.and_then(RoundOutcome::try_from_char)?;

		Ok(Self::from_outcome(opponent, outcome))
    }
}

impl Round {
	fn outcome(&self) -> RoundOutcome {
		match (self.you, self.opponent) {
			(Action::Rock, Action::Scissors) => RoundOutcome::Win,
			(Action::Paper, Action::Rock) => RoundOutcome::Win,
			(Action::Scissors, Action::Paper) => RoundOutcome::Win,

			(Action::Rock, Action::Rock) => RoundOutcome::Draw,
			(Action::Paper, Action::Paper) => RoundOutcome::Draw,
			(Action::Scissors, Action::Scissors) => RoundOutcome::Draw,
			
			(Action::Rock, Action::Paper) => RoundOutcome::Lose,
			(Action::Paper, Action::Scissors) => RoundOutcome::Lose,
			(Action::Scissors, Action::Rock) => RoundOutcome::Lose,
		}
	}

	fn from_outcome(opponent: Action, outcome: RoundOutcome) -> Self {
		let you = match (outcome, opponent) {
			(RoundOutcome::Win, Action::Rock) => Action::Paper,
			(RoundOutcome::Win, Action::Paper) => Action::Scissors,
			(RoundOutcome::Win, Action::Scissors) => Action::Rock,

			(RoundOutcome::Draw, Action::Rock) => Action::Rock,
			(RoundOutcome::Draw, Action::Paper) => Action::Paper,
			(RoundOutcome::Draw, Action::Scissors) => Action::Scissors,
			
			(RoundOutcome::Lose, Action::Rock) => Action::Scissors,
			(RoundOutcome::Lose, Action::Paper) => Action::Rock,
			(RoundOutcome::Lose, Action::Scissors) => Action::Paper,
		};

		Self { opponent, you }
	}

	fn score(&self) -> usize {
		self.you.score() + self.outcome().score()
	}
}

#[derive(Debug)]
pub(crate) struct ActionStrategyGuide {
	rounds: Vec<Round>,
}

impl Input for ActionStrategyGuide {
    type Error = StrategyGuideParseError;

    fn parse_str(data: &str) -> Result<Self, Self::Error> {
        data.trim()
			.split('\n')
			.map(|round| Round::try_from_action(round.trim()))
			.collect::<Result<Vec<_>, _>>()
			.map(|rounds| Self { rounds })
    }
}

impl ActionStrategyGuide {
	pub(crate) fn score(&self) -> usize {
		self.rounds.iter().map(Round::score).sum()
	}
}

#[derive(Debug)]
pub(crate) struct OutcomeStrategyGuide {
	rounds: Vec<Round>,
}

impl Input for OutcomeStrategyGuide {
    type Error = StrategyGuideParseError;

    fn parse_str(data: &str) -> Result<Self, Self::Error> {
        data.trim()
			.split('\n')
			.map(|round| Round::try_from_outcome(round.trim()))
			.collect::<Result<Vec<_>, _>>()
			.map(|rounds| Self { rounds })
    }
}

impl OutcomeStrategyGuide {
	pub(crate) fn score(&self) -> usize {
		self.rounds.iter().map(Round::score).sum()
	}
}