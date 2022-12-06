use crate::utils::{Day, Result};
use std::collections::HashSet;

pub struct Day6 {}

impl Day6 {
    const DAY: u8 = 6;

    pub fn part(input: String, window: usize) -> Result<String> {
        Ok((input
            .chars()
            .collect::<Vec<_>>()
            .windows(window)
            .position(|group| HashSet::<&char>::from_iter(group.iter()).len() == window)
            .ok_or("Result not found")?
            + window)
            .to_string())
    }
}

impl Day<String> for Day6 {
    fn day() -> u8 {
        Self::DAY
    }

    fn parse(input: &str) -> Result<String> {
        Ok(input.to_string())
    }

    fn part1(input: String) -> Result<String> {
        Day6::part(input, 4)
    }

    fn part2(input: String) -> Result<String> {
        Day6::part(input, 14)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn part1_test() -> Result<()> {
        assert_eq!(
            Day6::part1(Day6::parse("mjqjpqmgbljsphdztnvjfqwrcgsmlb")?)?,
            "7"
        );
        assert_eq!(
            Day6::part1(Day6::parse("bvwbjplbgvbhsrlpgdmjqwftvncz")?)?,
            "5"
        );
        assert_eq!(
            Day6::part1(Day6::parse("nppdvjthqldpwncqszvftbrmjlhg")?)?,
            "6"
        );
        assert_eq!(
            Day6::part1(Day6::parse("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")?)?,
            "10"
        );
        assert_eq!(
            Day6::part1(Day6::parse("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")?)?,
            "11"
        );
        Ok(())
    }

    #[test]
    fn part2_test() -> Result<()> {
        assert_eq!(
            Day6::part2(Day6::parse("mjqjpqmgbljsphdztnvjfqwrcgsmlb")?)?,
            "19"
        );
        assert_eq!(
            Day6::part2(Day6::parse("bvwbjplbgvbhsrlpgdmjqwftvncz")?)?,
            "23"
        );
        assert_eq!(
            Day6::part2(Day6::parse("nppdvjthqldpwncqszvftbrmjlhg")?)?,
            "23"
        );
        assert_eq!(
            Day6::part2(Day6::parse("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")?)?,
            "29"
        );
        assert_eq!(
            Day6::part2(Day6::parse("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")?)?,
            "26"
        );
        Ok(())
    }
}
