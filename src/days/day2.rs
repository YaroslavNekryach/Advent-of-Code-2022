use crate::utils::{Day, Result};

const DAY: u8 = 2;

pub struct Day2 {}

impl Day2 {
    fn get_score_part_1(round: &String) -> u64 {
        match round.as_str() {
            "A X" => 1 + 3,
            "A Y" => 2 + 6,
            "A Z" => 3 + 0,
            "B X" => 1 + 0,
            "B Y" => 2 + 3,
            "B Z" => 3 + 6,
            "C X" => 1 + 6,
            "C Y" => 2 + 0,
            "C Z" => 3 + 3,
            _ => panic!("Invalid round"),
        }
    }

    fn get_score_part_2(round: &String) -> u64 {
        match round.as_str() {
            "A X" => 3 + 0,
            "A Y" => 1 + 3,
            "A Z" => 2 + 6,
            "B X" => 1 + 0,
            "B Y" => 2 + 3,
            "B Z" => 3 + 6,
            "C X" => 2 + 0,
            "C Y" => 3 + 3,
            "C Z" => 1 + 6,
            _ => panic!("Invalid round"),
        }
    }
}

impl Day<Vec<String>> for Day2 {
    fn day() -> u8 {
        DAY
    }

    fn parse(input: &str) -> Result<Vec<String>> {
        Ok(input.lines().map(str::to_string).collect())
    }

    fn part1(input: Vec<String>) -> Result<String> {
        Ok(input
            .iter()
            .map(Self::get_score_part_1)
            .sum::<u64>()
            .to_string())
    }

    fn part2(input: Vec<String>) -> Result<String> {
        Ok(input
            .iter()
            .map(Self::get_score_part_2)
            .sum::<u64>()
            .to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn part1_test() {
        let parsed_input = Day2::parse(INPUT).unwrap();
        let result = Day2::part1(parsed_input.clone()).unwrap();
        assert_eq!(&result, "15")
    }

    #[test]
    fn part2_test() {
        let parsed_input = Day2::parse(INPUT).unwrap();
        let result = Day2::part2(parsed_input.clone()).unwrap();
        assert_eq!(&result, "12")
    }

    const INPUT: &str = "A Y
B X
C Z";
}
