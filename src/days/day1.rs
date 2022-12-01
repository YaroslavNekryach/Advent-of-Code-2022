use crate::utils::{get_input, split_by_empty_line, split_lines};
use std::str::FromStr;
const DAY: u8 = 1;

pub fn exec() {
    let input = get_input(DAY).unwrap();
    let elves = split_by_empty_line(&input)
        .iter()
        .map(|elf| {
            split_lines(elf)
                .iter()
                .map(|v| u64::from_str(v.as_str()).unwrap())
                .collect()
        })
        .collect();

    let part1_result = part1(&elves);
    println!("Part 1 result {}", part1_result);
    let part1_result = part2(&elves);
    println!("Part 2 result {}", part1_result);
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
