extern crate regex;

use crate::util::*;
use std::error::Error;
use regex::Regex;

struct PasswordEntry {
    min: usize,
    max: usize,
    character: char,
    password: String,
}

#[derive(Debug, Clone)]
struct ParsedInvalidFormat;
impl Error for ParsedInvalidFormat {}
impl std::fmt::Display for ParsedInvalidFormat {
	fn fmt(&self, formatter: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error>{
		write!(formatter, "cannot parse invalid format")
	}
}

impl PasswordEntry {
    fn parse(entry: &str) -> Result<PasswordEntry, Box<dyn Error>> {
        let details = Regex::new(r"^([0-9]+)-([0-9]+)\s+([^\s]):\s+([^\s]+)\s*$")
            .unwrap()
            .captures(entry)
            .ok_or(ParsedInvalidFormat)?;

        match details.len() {
            5 => Ok(PasswordEntry {
                min: details.get(1).ok_or(ParsedInvalidFormat)?.as_str().parse()?,
                max: details.get(2).ok_or(ParsedInvalidFormat)?.as_str().parse()?,
                character: details.get(3).ok_or(ParsedInvalidFormat)?.as_str().parse()?,
                password: details.get(4).ok_or(ParsedInvalidFormat)?.as_str().to_owned(),
            }),
            _ => Err(ParsedInvalidFormat.into()),
        }
    }

    fn validate(&self) -> bool {
        let count = self.password.matches(self.character).count();
        return count >= self.min && count <= self.max;
    }
}

fn solve() -> usize {
    let passwords = read_file("resources/day2/passwords.txt", |s| PasswordEntry::parse(s).unwrap());
    passwords.iter().filter(|p| p.validate()).count()
}

pub fn print_solution() {
	print_day(2);
	let valid_password_count = solve();
    println!("{} {} valid", valid_password_count, "password".to_owned().pluralize(valid_password_count));
}