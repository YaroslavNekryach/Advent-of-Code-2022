extern crate core;

use crate::days::day5::Day5;
use crate::days::day6::Day6;
use crate::utils::Day;

mod days;
mod utils;

fn main() {
    if false {
        all_days()
    }
    Day6::exec().unwrap();
}

fn all_days() {
    days::day3::exec();
    days::day4::exec();
}
