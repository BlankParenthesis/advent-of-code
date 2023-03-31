extern crate itertools;

use crate::util::*;
use itertools::Itertools;

fn solve() -> Option<Vec<isize>> {
    let expense_report = read_file("resources/day1/expense_report.txt", |s| s.parse::<isize>().unwrap());
    
    expense_report
        .iter()
		.combinations(2)
		.map(|pair| pair.iter().map(|i| **i).collect::<Vec<isize>>())
        .find(|pair| pair.iter().sum::<isize>() == 2020)
}

pub fn print_solution() {
    print_day(1);

    match solve() {
        Some(pair) => println!("{}, {}", pair[0], pair[1]),
        None => println!("No valid pair"),
    }
}