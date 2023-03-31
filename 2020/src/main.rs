mod day1;
mod day2;
mod day3;
mod day4;
mod util;

fn main() {
    println!("{}", u8::from_str_radix("1", 16).unwrap());
    day1::print_solution();
    day2::print_solution();
    day3::print_solution();
    day4::print_solution();
}
