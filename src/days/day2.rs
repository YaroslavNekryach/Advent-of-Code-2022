use crate::utils::{get_input, print_result};
use std::str::Lines;

const DAY: u8 = 2;

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
    input.map(get_score_part_1).sum::<u64>().to_string()
}

fn part2(input: Lines) -> String {
    input.map(get_score_part_2).sum::<u64>().to_string()
}

fn get_score_part_1(round: &str) -> u64 {
    match round {
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

fn get_score_part_2(round: &str) -> u64 {
    match round {
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

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn part1_test() {
        let parsed_input = parse(INPUT);
        let result = part1(parsed_input.clone());
        assert_eq!(&result, "15")
    }

    #[test]
    fn part2_test() {
        let parsed_input = parse(INPUT);
        let result = part2(parsed_input.clone());
        assert_eq!(&result, "12")
    }

    const INPUT: &str = "A Y
B X
C Z";
}
