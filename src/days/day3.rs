use crate::utils::{Day, Result};

pub struct Day3 {}

impl Day3 {
    const DAY: u8 = 3;

    fn get_score(char: char) -> u64 {
        if char.is_lowercase() {
            char as u64 - 96
        } else {
            char as u64 - 38
        }
    }
}

impl Day<Vec<String>> for Day3 {
    fn day() -> u8 {
        Self::DAY
    }

    fn parse(input: &str) -> Result<Vec<String>> {
        Ok(input.lines().map(str::to_string).collect())
    }

    fn part1(input: Vec<String>) -> Result<String> {
        Ok(input
            .iter()
            .map(|rucksack| {
                let (part1, part2) = rucksack.split_at(rucksack.len() / 2);
                part1
                    .chars()
                    .find(|c| part2.contains(&c.to_string()))
                    .unwrap()
            })
            .map(Self::get_score)
            .sum::<u64>()
            .to_string())
    }

    fn part2(input: Vec<String>) -> Result<String> {
        Ok(input
            .windows(3)
            .step_by(3)
            .map(|ru| {
                let ru1 = &ru[0];
                let ru2 = &ru[1];
                let ru3 = &ru[2];
                ru1.chars()
                    .find(|c| ru2.contains(&c.to_string()) & ru3.contains(&c.to_string()))
                    .unwrap()
            })
            .map(Self::get_score)
            .sum::<u64>()
            .to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn part1_test() {
        let parsed_input = Day3::parse(INPUT).unwrap();
        let result = Day3::part1(parsed_input).unwrap();
        assert_eq!(&result, "157")
    }

    #[test]
    fn part2_test() {
        let parsed_input = Day3::parse(INPUT).unwrap();
        let result = Day3::part2(parsed_input).unwrap();
        assert_eq!(&result, "70")
    }

    const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
}
