mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;
mod day_9;
mod day_10;

fn main() {
    println!("day 1: {:?}", day_1::solve(
        day_1::parse_input(include_str!("day_1_input.txt")).unwrap()
    ));

    println!("day 2: {:?}", day_2::solve(
        day_2::parse_input(include_str!("day_2_input.txt")).unwrap()
    ));

    println!("day 3: {:?}", day_3::solve(
        day_3::parse_input(include_str!("day_3_input.txt")).unwrap()
    ));

    println!("day 4: {:?}", day_4::solve(
        day_4::parse_input(include_str!("day_4_input.txt")).unwrap()
    ));

    println!("day 5: {:?}", day_5::solve(
        day_5::parse_input(include_str!("day_5_input.txt")).unwrap()
    ));

    println!("day 6: {:?}", day_6::solve(
        day_6::parse_input(include_str!("day_6_input.txt")).unwrap()
    ));

    println!("day 7: {:?}", day_7::solve(
        day_7::parse_input(include_str!("day_7_input.txt")).unwrap()
    ));

    println!("day 8: {:?}", day_8::solve(
        day_8::parse_input(include_str!("day_8_input.txt")).unwrap()
    ));

    println!("day 9: {:?}", day_9::solve(
        day_9::parse_input(include_str!("day_9_input.txt")).unwrap()
    ));

    println!("day 10: {:?}", day_10::solve(
        day_10::parse_input(include_str!("day_10_input.txt")).unwrap()
    ));
}
