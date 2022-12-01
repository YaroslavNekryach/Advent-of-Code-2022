use crate::utils::{get_input, print_result, split_by_empty_line, split_lines};
use std::str::FromStr;

const DAY: u8 = 1;

pub fn exec() {
    let input = get_input(DAY).unwrap();
    let parsed_input = parse(&input);
    print_result(1, &part1(&parsed_input));
    print_result(2, &part2(&parsed_input));
}

fn parse(input: &str) -> Vec<Vec<u64>> {
    split_by_empty_line(input)
        .iter()
        .map(|elf| {
            split_lines(elf)
                .iter()
                .map(|v| u64::from_str(v.as_str()).unwrap())
                .collect()
        })
        .collect()
}

fn part1(input: &Vec<Vec<u64>>) -> String {
    total_elf_cal(input).iter().max().unwrap().to_string()
}

fn part2(input: &Vec<Vec<u64>>) -> String {
    let mut elf_cal = total_elf_cal(input);
    elf_cal.sort();
    elf_cal.iter().rev().take(3).sum::<u64>().to_string()
}

fn total_elf_cal(input: &Vec<Vec<u64>>) -> Vec<u64> {
    input.iter().map(|elf| elf.iter().sum()).collect()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn part1_test() {
        let parsed_input = parse(INPUT);
        let result = part1(&parsed_input);
        assert_eq!(&result, "24000")
    }

    #[test]
    fn part2_test() {
        let parsed_input = parse(INPUT);
        let result = part2(&parsed_input);
        assert_eq!(&result, "45000")
    }

    const INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";
}
