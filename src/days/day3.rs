use crate::utils::{get_input, print_result};
use std::str::Lines;

const DAY: u8 = 3;

pub fn exec() {
    let input = get_input(DAY).unwrap();
    let parsed_input = parse(&input);
    print_result(1, &part1(parsed_input.clone()));
    print_result(2, &part2(parsed_input.clone()));
}

fn parse(input: &str) -> Lines {
    input.lines()
}

fn part1(input: Lines) -> String {
    input
        .map(|rucksack| {
            let (part1, part2) = rucksack.split_at(rucksack.len() / 2);
            part1
                .chars()
                .find(|c| part2.contains(&c.to_string()))
                .unwrap()
        })
        .map(get_score)
        .sum::<u64>()
        .to_string()
}

fn part2(input: Lines) -> String {
    input
        .collect::<Vec<&str>>()
        .windows(3)
        .step_by(3)
        .map(|ru| {
            let ru1 = ru[0];
            let ru2 = ru[1];
            let ru3 = ru[2];
            ru1.chars()
                .find(|c| ru2.contains(&c.to_string()) & ru3.contains(&c.to_string()))
                .unwrap()
        })
        .map(get_score)
        .sum::<u64>()
        .to_string()
}

fn get_score(char: char) -> u64 {
    if char.is_lowercase() {
        char as u64 - 96
    } else {
        char as u64 - 38
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn part1_test() {
        let parsed_input = parse(INPUT);
        let result = part1(parsed_input.clone());
        assert_eq!(&result, "157")
    }

    #[test]
    fn part2_test() {
        let parsed_input = parse(INPUT);
        let result = part2(parsed_input.clone());
        assert_eq!(&result, "70")
    }

    const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
}
