use crate::util::*;
use std::error::Error;
use std::collections::HashSet;
use std::hash::Hash;
use std::hash::Hasher;

type Year = isize;
type Color = (u8, u8, u8);

#[derive(Eq)]
enum PassportField {
	BirthYear(Year),
	IssueYear(Year),
	ExpirationYear(Year),
	Height(String),
	HairColor(Color),
	EyeColor(String),
	PassportId(String),
	CountryId(String),
}

impl PartialEq<PassportField> for PassportField {
	fn eq(&self, other: &PassportField) -> bool {
		match self {
			PassportField::BirthYear(_) => match other { PassportField::BirthYear(_) => true, _ => false }
			PassportField::IssueYear(_) => match other { PassportField::IssueYear(_) => true, _ => false }
			PassportField::ExpirationYear(_) => match other { PassportField::ExpirationYear(_) => true, _ => false }
			PassportField::Height(_) => match other { PassportField::Height(_) => true, _ => false }
			PassportField::HairColor(_) => match other { PassportField::HairColor(_) => true, _ => false }
			PassportField::EyeColor(_) => match other { PassportField::EyeColor(_) => true, _ => false }
			PassportField::PassportId(_) => match other { PassportField::PassportId(_) => true, _ => false }
			PassportField::CountryId(_) => match other { PassportField::CountryId(_) => true, _ => false }
		}
	}
}

impl Hash for PassportField {
    fn hash<H: Hasher>(&self, state: &mut H) {
		state.write_u8(match self {
			PassportField::BirthYear(_) => 0,
			PassportField::IssueYear(_) => 1,
			PassportField::ExpirationYear(_) => 2,
			PassportField::Height(_) => 3,
			PassportField::HairColor(_) => 4,
			PassportField::EyeColor(_) => 5,
			PassportField::PassportId(_) => 6,
			PassportField::CountryId(_) => 7,
		});
	}
}

fn parse_color_part(hex: &str) -> Result<u8, Box<dyn Error>> {
	Ok(u8::from_str_radix(hex, 16)?)
}

fn parse_color(prefixed_hex: &str) -> Result<Color, Box<dyn Error>> {
	let hex = prefixed_hex.trim_start_matches("#");
	Ok((
		parse_color_part(hex.get(0..2).ok_or("parsed color too short")?)?,
		parse_color_part(hex.get(2..4).ok_or("parsed color too short")?)?,
		parse_color_part(hex.get(4..6).ok_or("parsed color too short")?)?,
	))
}

impl PassportField {
	fn parse(entry: &str) -> Result<PassportField, Box<dyn Error>> {
		let parts: Vec<&str> = entry.split(":").collect();
		match parts.len() {
			2 => match *parts.get(0).unwrap() {
				"byr" => Ok(PassportField::BirthYear(parts.get(1).unwrap().parse()?)),
				"iyr" => Ok(PassportField::IssueYear(parts.get(1).unwrap().parse()?)),
				"eyr" => Ok(PassportField::ExpirationYear(parts.get(1).unwrap().parse()?)),
				"hgt" => Ok(PassportField::Height((*parts.get(1).unwrap()).to_owned())),
				"hcl" => Ok(PassportField::HairColor(parse_color(parts.get(1).unwrap())?)),
				"ecl" => Ok(PassportField::EyeColor((*parts.get(1).unwrap()).to_owned())),
				"pid" => Ok(PassportField::PassportId((*parts.get(1).unwrap()).to_owned())),
				"cid" => Ok(PassportField::CountryId((*parts.get(1).unwrap()).to_owned())),
				key => Err(format!("unrecognised password field {}", key).into()),
			},
			_ => Err("passport field had incorrect number of parts".into()),
		}
	}
}

struct Passport {
	fields: HashSet<PassportField>,
}

impl Passport {
	fn parse(entry: &str) -> Result<Passport, Box<dyn Error>> {
		Ok(Passport {
			fields: entry.split_ascii_whitespace()
				.map(|e| PassportField::parse(e))
				.collect::<Result<HashSet<PassportField>, Box<dyn Error>>>()?,
		})
	}

	fn valid(&self) -> bool {
		self.fields.contains(&PassportField::BirthYear(Default::default()))
		&& self.fields.contains(&PassportField::IssueYear(Default::default()))
		&& self.fields.contains(&PassportField::ExpirationYear(Default::default()))
		&& self.fields.contains(&PassportField::Height(Default::default()))
		&& self.fields.contains(&PassportField::HairColor(Default::default()))
		&& self.fields.contains(&PassportField::EyeColor(Default::default()))
		&& self.fields.contains(&PassportField::PassportId(Default::default()))
		//&& self.fields.contains(&PassportField::CountryId(Default::default()))
	}
}

fn solve() -> usize {
	let passports = read_file_delimiter("resources/day4/passports.txt", "\n\n", |e| Passport::parse(e).unwrap());
	passports.iter().filter(|p| p.valid()).count()
}

pub fn print_solution() {
	print_day(4);
	let valid_passport_count = solve();
	println!("{} {} valid", valid_passport_count, "passport".to_owned().pluralize(valid_passport_count));
}