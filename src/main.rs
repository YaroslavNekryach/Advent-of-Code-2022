extern crate core;

use crate::days::day5::Day5;
use crate::utils::Day;

mod days;
mod utils;

fn main() {
    if false {
        all_days()
    }
    Day5::exec().unwrap();
}

fn all_days() {
    days::day2::exec();
    days::day3::exec();
    days::day4::exec();
}
