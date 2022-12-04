use crate::utils::{parse_int, Day, Result};

pub struct Day1 {}

impl Day1 {
    const DAY: u8 = 1;

    fn total_elf_cal(input: &[Vec<u64>]) -> Vec<u64> {
        input.iter().map(|elf| elf.iter().sum()).collect()
    }
}

impl Day<Vec<Vec<u64>>> for Day1 {
    fn day() -> u8 {
        Self::DAY
    }

    fn parse(input: &str) -> Result<Vec<Vec<u64>>> {
        Ok(input
            .split("\n\n")
            .map(|elf| elf.lines().map(parse_int).collect())
            .collect())
    }

    fn part1(input: Vec<Vec<u64>>) -> Result<String> {
        Ok(Self::total_elf_cal(&input)
            .iter()
            .max()
            .unwrap()
            .to_string())
    }

    fn part2(input: Vec<Vec<u64>>) -> Result<String> {
        let mut elf_cal = Self::total_elf_cal(&input);
        elf_cal.sort_unstable();
        Ok(elf_cal.iter().rev().take(3).sum::<u64>().to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn part1_test() -> Result<()> {
        let parsed_input = Day1::parse(INPUT)?;
        let result = Day1::part1(parsed_input)?;
        assert_eq!(&result, "24000");
        Ok(())
    }

    #[test]
    fn part2_test() -> Result<()> {
        let parsed_input = Day1::parse(INPUT)?;
        let result = Day1::part2(parsed_input)?;
        assert_eq!(&result, "45000");
        Ok(())
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
