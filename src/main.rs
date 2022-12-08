extern crate core;

use crate::days::day1::Day1;
use crate::days::day2::Day2;
use crate::days::day3::Day3;
use crate::days::day4::Day4;
use crate::days::day5::Day5;
use crate::days::day6::Day6;
use crate::days::day8::Day8;
use crate::utils::Day;

mod days;
mod utils;

fn main() {
    // Day1::exec().unwrap();
    // Day2::exec().unwrap();
    // Day3::exec().unwrap();
    // Day4::exec().unwrap();
    // Day5::exec().unwrap();
    // Day6::exec().unwrap();
    // Day7::exec().unwrap();
    Day8::exec().unwrap();
}
