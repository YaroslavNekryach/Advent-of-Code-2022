extern crate core;

mod days;
mod utils;

fn main() {
    if false {
        all_days()
    }
    days::day4::exec();
}

fn all_days() {
    days::day1::exec();
    days::day2::exec();
    days::day3::exec();
    days::day4::exec();
}
