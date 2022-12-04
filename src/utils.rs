use reqwest::blocking::Client;
use reqwest::Error;

const SESSION: &str = "53616c7465645f5f7998bb14334539d997cfee9321c8e5d3f72f056853a51a004b29784458b5208668ec450c9083a997dbe65a1ef7927c85173a2ee31e6f7065";
const YEAR: &str = "2022";
const URL: &str = "https://adventofcode.com";

pub fn get_input(day: u8) -> Result<String, Error> {
    let client = Client::new();
    let url = format!("{URL}/{YEAR}/day/{day}/input");
    let res = client
        .get(url)
        .header("Cookie", format!("session={SESSION}"))
        .send()?;

    let text = res.text()?;
    Ok(text.strip_suffix("\n").unwrap_or(&text).to_string())
}

pub fn print_result(part: u8, result: &str) {
    println!("\x1b[34mPart {}:\x1b[0m \x1b[93m{}\x1b[0m", part, result);
}

pub trait SplitString {
    fn split_by(&self, terminator: char) -> Result<(&str, &str), std::fmt::Error>;
}

impl SplitString for &str {
    fn split_by(&self, terminator: char) -> Result<(&str, &str), std::fmt::Error> {
        let split: Vec<&str> = self.split(terminator).collect();
        if split.len() != 2 {
            return Err(std::fmt::Error);
        }
        Ok((split[0], split[1]))
    }
}
