extern crate core;

use crate::days::day1::Day1;
use crate::utils::Day;

mod days;
mod utils;

fn main() {
    if false {
        all_days()
    }
    Day1::exec().unwrap();
}

fn all_days() {
    days::day2::exec();
    days::day3::exec();
    days::day4::exec();
}
