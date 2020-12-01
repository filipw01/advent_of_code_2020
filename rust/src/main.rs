mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

use std::env::args;

fn main() {
    let day = args().nth(1).unwrap();
    match day.as_str() {
        "1" => day1::run(),
        "2" => day2::run(),
        "3" => day3::run(),
        "4" => day4::run(),
        "5" => day5::run(),
        _ => panic!("Wrong day")
    }
}
