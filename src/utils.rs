use reqwest::blocking::Client;
use std::error::Error;
use std::result;
use std::time::{Duration, Instant};

const SESSION: &str = "53616c7465645f5f7998bb14334539d997cfee9321c8e5d3f72f056853a51a004b29784458b5208668ec450c9083a997dbe65a1ef7927c85173a2ee31e6f7065";
const YEAR: &str = "2022";
const URL: &str = "https://adventofcode.com";

pub fn get_input(day: u8) -> Result<String> {
    let client = Client::new();
    let url = format!("{URL}/{YEAR}/day/{day}/input");
    let res = client
        .get(url)
        .header("Cookie", format!("session={SESSION}"))
        .send()?;

    let text = res.text()?;
    Ok(text.strip_suffix("\n").unwrap_or(&text).to_string())
}

pub fn print_result(part: u8, result: &str, duration: Duration) {
    println!(
        "\x1b[34mPart {}:\x1b[0m \x1b[93m{}\x1b[0m ({:.2?})",
        part, result, duration
    );
}

pub fn parse_int(s: &str) -> u64 {
    s.parse().unwrap()
}

pub trait SplitString {
    fn split_by(&self, terminator: &str) -> Result<(&str, &str)>;
}

impl SplitString for &str {
    fn split_by(&self, terminator: &str) -> Result<(&str, &str)> {
        let split: Vec<&str> = self.split(terminator).collect();
        if split.len() != 2 {
            return Err(std::fmt::Error.into());
        }
        Ok((split[0], split[1]))
    }
}

pub type Result<T> = result::Result<T, Box<dyn Error>>;

pub trait Day<T: Clone> {
    fn exec() -> Result<()> {
        println!("Day {}:", Self::day());
        let input = get_input(Self::day())?;
        let parsed_input = Self::parse(&input)?;
        let now = Instant::now();
        print_result(1, &Self::part1(parsed_input.clone())?, now.elapsed());
        let now = Instant::now();
        print_result(2, &Self::part2(parsed_input)?, now.elapsed());
        Ok(())
    }

    fn day() -> u8;

    fn parse(input: &str) -> Result<T>;
    fn part1(input: T) -> Result<String>;
    fn part2(input: T) -> Result<String>;
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Pos<T> {
    pub x: T,
    pub y: T,
}

impl<T> Pos<T> {
    pub fn new(x: T, y: T) -> Self {
        Pos { x, y }
    }
}

pub trait PosComponent3D {
    fn x(&self) -> u64;
    fn y(&self) -> u64;
    fn z(&self) -> u64;
}

impl PosComponent3D for (u64, u64, u64) {
    fn x(&self) -> u64 {
        self.0
    }

    fn y(&self) -> u64 {
        self.1
    }

    fn z(&self) -> u64 {
        self.2
    }
}

pub trait PosComponent2D {
    fn x(&self) -> u64;
    fn y(&self) -> u64;
}

impl PosComponent2D for (u64, u64) {
    fn x(&self) -> u64 {
        self.0
    }

    fn y(&self) -> u64 {
        self.1
    }
}
