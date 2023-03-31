use nom::{bytes::complete::tag, sequence::{preceded, tuple, delimited, separated_pair}, combinator::{map_res, map}, character::complete::{digit1, alpha1, multispace1}, multi::{separated_list0, separated_list1}, Parser as NomParser};
use crate::take_positive_number;

#[derive(Debug)]
pub struct Blueprint {
	pub id: usize,
	ore: usize,
	clay: usize,
	obsidian: (usize, usize),
	geode: (usize, usize),
}

#[derive(Debug, Clone, Copy)]
enum Resource {
	Ore,
	Clay,
	Obsidian,
	Geode,
}

#[derive(Debug, Clone)]
pub struct Resources {
	ore: usize,
	ore_bots: usize,
	clay: usize,
	clay_bots: usize,
	obsidian: usize,
	obsidian_bots: usize,
	geodes: usize,
	geode_bots: usize,
}

impl Resources {
	fn tick(&self, count: usize) -> Self {
		Self {
			ore: self.ore + self.ore_bots * count,
			ore_bots: self.ore_bots,
			clay: self.clay + self.clay_bots * count,
			clay_bots: self.clay_bots,
			obsidian: self.obsidian + self.obsidian_bots * count,
			obsidian_bots: self.obsidian_bots,
			geodes: self.geodes + self.geode_bots * count,
			geode_bots: self.geode_bots,
		}
	}

	fn etas(&self, blueprint: &Blueprint) -> (f32, f32, f32, f32) {
		let eta_ore = blueprint.ore.saturating_sub(self.ore) as f32 / self.ore_bots as f32;
		let eta_clay = blueprint.clay.saturating_sub(self.ore) as f32 / self.ore_bots as f32;
		let eta_obsidian = f32::max(
			blueprint.obsidian.0.saturating_sub(self.ore) as f32 / self.ore_bots as f32,
			blueprint.obsidian.1.saturating_sub(self.clay) as f32 / self.clay_bots as f32,
		);
		let eta_geode = f32::max(
			blueprint.geode.0.saturating_sub(self.ore) as f32 / self.ore_bots as f32,
			blueprint.geode.1.saturating_sub(self.obsidian) as f32 / self.obsidian_bots as f32,
		);

		(
			eta_ore.ceil() + 1_f32,
			eta_clay.ceil() + 1_f32,
			eta_obsidian.ceil() + 1_f32,
			eta_geode.ceil() + 1_f32,
		)
	}

	fn tier(&self) -> Resource {
		if self.obsidian_bots > 0 {
			Resource::Geode
		} else if self.clay_bots > 0 {
			Resource::Obsidian
		} else {
			Resource::Clay
		}
	}
	
	fn can_build(&self, blueprint: &Blueprint, bot: Resource) -> bool {
		match bot {
			Resource::Ore => self.ore >= blueprint.ore,
			Resource::Clay => self.ore >= blueprint.clay,
			Resource::Obsidian => self.ore >= blueprint.obsidian.0 && self.clay >= blueprint.obsidian.1,
			Resource::Geode => self.ore >= blueprint.geode.0 && self.obsidian >= blueprint.geode.1,
		}
	}

	fn build(&self, blueprint: &Blueprint, bot: Resource) -> Self {
		self.can_build(blueprint, bot)
			.then_some(())
			.unwrap_or_else(|| {
				println!();
				println!();
				panic!("Failed to build {:?} with {:?}, {:?}", bot, (self.ore, self.clay, self.obsidian, self.geodes), (self.ore_bots, self.clay_bots, self.obsidian_bots, self.geode_bots));
			});
		
		let mut new = self.clone();
		match bot {
			Resource::Ore => {
				new.ore -= blueprint.ore;
				new.ore_bots += 1;
			},
			Resource::Clay => {
				new.ore -= blueprint.clay;
				new.clay_bots += 1;
			},
			Resource::Obsidian => {
				new.ore -= blueprint.obsidian.0;
				new.clay -= blueprint.obsidian.1;
				new.obsidian_bots += 1;
			},
			Resource::Geode => {
				new.ore -= blueprint.geode.0;
				new.obsidian -= blueprint.geode.1;
				new.geode_bots += 1;
			},
		}

		new
	}
}

impl Default for Resources {
	fn default() -> Self {
		Self { ore: 0, ore_bots: 1, clay: 0, clay_bots: 0, obsidian: 0, obsidian_bots: 0, geodes: 0, geode_bots: 0 }
	}
}

impl Blueprint {
	pub fn parse(input: &str) -> nom::IResult<&str, Self> {
		map(tuple((
			delimited(tag("Blueprint "), take_positive_number, tag(":").and(multispace1)),
			delimited(tag("Each ore robot costs "), take_positive_number, tag(" ore.").and(multispace1)),
			delimited(tag("Each clay robot costs "), take_positive_number, tag(" ore.").and(multispace1)),
			delimited(tag("Each obsidian robot costs "), separated_pair(take_positive_number, tag(" ore and "), take_positive_number), tag(" clay.").and(multispace1)),
			delimited(tag("Each geode robot costs "), separated_pair(take_positive_number, tag(" ore and "), take_positive_number), tag(" obsidian.")),
		)), |(id, ore, clay, obsidian, geode)| {
			Self {
				id,
				ore,
				clay,
				obsidian,
				geode,
			}
		})(input)
	}

	pub fn parse_all(input: &str) -> nom::IResult<&str, Vec<Self>> {
		separated_list1(multispace1, Blueprint::parse)(input)
	}

	pub fn find_max_geodes(&self, minutes: isize) -> usize {
		let mut resources = Resources::default();

		let mut minutes_left = minutes;
		while minutes_left > 0 {
			let best_possible = resources.tier();
			let (eta_ore, eta_clay, eta_obsidian, eta_geode) = resources.etas(self);

			let snapshot_resources = (resources.ore, resources.clay, resources.obsidian, resources.geodes);
			let snapshot_bots = (resources.ore_bots, resources.clay_bots, resources.obsidian_bots, resources.geode_bots);
			
			// select bot to build which minimizes the best possible bot eta
			let (eta, resource) = match best_possible {
				Resource::Clay => {
					print!("{} (Clay): {:?}, {:?}", minutes_left, snapshot_resources, snapshot_bots);
					let ore_resources = resources
						.tick(eta_ore as usize)
						.build(self, Resource::Ore);

					let ore_eta = ore_resources.etas(self).1 + eta_ore;

					if ore_eta <= eta_clay  {
						(eta_ore, Resource::Ore)
					} else {
						(eta_clay, Resource::Clay)
					}
				},
				Resource::Obsidian => {
					print!("{} (Obsidian): {:?}, {:?}", minutes_left, snapshot_resources, snapshot_bots);

					let ore_resources = resources
						.tick(eta_ore as usize)
						.build(self, Resource::Ore);

					let clay_resources = resources
						.tick(eta_clay as usize)
						.build(self, Resource::Clay);

					let ore_eta = ore_resources.etas(self).2 + eta_ore;
					let clay_eta = clay_resources.etas(self).2 + eta_clay;

					if ore_eta <= eta_obsidian  {
						if clay_eta <= ore_eta {
							(eta_clay, Resource::Clay)
						} else {
							(eta_ore, Resource::Ore)
						}
					} else {
						(eta_obsidian, Resource::Obsidian)
					}
				},
				Resource::Geode => {
					print!("{} (Geode): {:?}, {:?}", minutes_left, snapshot_resources, snapshot_bots);

					let ore_resources = resources
						.tick(eta_ore as usize)
						.build(self, Resource::Ore);

					let clay_resources = resources
						.tick(eta_clay as usize)
						.build(self, Resource::Clay);

					let obsidian_resources = resources
						.tick(eta_obsidian as usize)
						.build(self, Resource::Obsidian);

					let ore_eta = ore_resources.etas(self).3 + eta_ore;
					let clay_eta = clay_resources.etas(self).3 + eta_clay;
					let obsidian_eta = obsidian_resources.etas(self).3 + eta_obsidian;

					if ore_eta <= eta_geode  {
						if clay_eta <= ore_eta {
							if obsidian_eta <= clay_eta {
								(eta_obsidian, Resource::Obsidian)
							} else {
								(eta_clay, Resource::Clay)
							}
						} else {
							(eta_ore, Resource::Ore)
						}
					} else {
						(eta_geode, Resource::Geode)
					}
				},
				_ => panic!(),
			};

			minutes_left -= eta as isize;
			if minutes_left > 0 {
				print!(", ticking {} to build {:?}", eta as usize, resource);
				println!();
				resources = resources
					.tick(eta as usize)
					.build(self, resource);
			}
		}

		resources.geodes
	}

	//pub fn find_max_geodes(&self, resources: Option<Resources>, minutes: isize) -> usize {
	//	if minutes <= 0 {
	//		resources.unwrap_or_default().geodes
	//	} else {
	//		let mut results = Vec::new();
	//		let resources = resources.unwrap_or_default();
	
	//		if resources.ore >= self.ore {
	//			let mut resources = resources.tick(1);
	//			resources.ore -= self.ore;
	//			resources.ore_bots += 1;
	//			results.push(self.find_max_geodes(Some(resources), minutes - 1));
	//		}

	//		if resources.ore >= self.clay {
	//			let mut resources = resources.tick(1);
	//			resources.ore -= self.clay;
	//			resources.clay_bots += 1;
	//			results.push(self.find_max_geodes(Some(resources), minutes - 1));
	//		}

	//		if resources.ore >= self.obsidian.0 && resources.clay > self.obsidian.1 {
	//			let mut resources = resources.tick(1);
	//			resources.ore -= self.obsidian.0;
	//			resources.clay -= self.obsidian.1;
	//			resources.obsidian_bots += 1;
	//			results.push(self.find_max_geodes(Some(resources), minutes - 1));
	//		}

	//		if resources.ore >= self.geode.0 && resources.obsidian > self.geode.1 {
	//			let mut resources = resources.tick(1);
	//			resources.ore -= self.geode.0;
	//			resources.obsidian -= self.geode.1;
	//			resources.geodes += (minutes as usize).saturating_sub(1);
	//			results.push(self.find_max_geodes(Some(resources), minutes - 1));
	//		}


	//		let tier = if resources.obsidian_bots > 0 {
	//			4
	//		} else if resources.clay_bots > 0 {
	//			3
	//		} else {
	//			2
	//		};
	//		// only do nothing if there's something we could do later
	//		if results.len() < tier {
	//			results.push(self.find_max_geodes(Some(resources.tick(1)), minutes - 1));
	//		}

	//		results.into_iter().max().unwrap_or(0)
	//	}
	//}
}