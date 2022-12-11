use crate::days::day11::monkey::Monkey;
use crate::utils::{parse_int, Day, Result};
use std::time::Instant;

pub struct Day11 {}

impl Day11 {
    const DAY: u8 = 11;
}

impl Day<Vec<Monkey>> for Day11 {
    fn day() -> u8 {
        Self::DAY
    }

    fn parse(input: &str) -> Result<Vec<Monkey>> {
        Ok(input.split("\n\n").map(Monkey::from_string).collect())
    }

    fn part1(mut input: Vec<Monkey>) -> Result<String> {
        let now = Instant::now();
        for _ in 0..20 {
            for monkey_i in 0..input.len() {
                while let Some((item, monkey)) = input[monkey_i].test(None) {
                    input[monkey].add_item(item);
                }
            }
        }
        input.sort_by(|a, b| a.inspections.cmp(&b.inspections));
        let elapsed = now.elapsed();
        println!("Part 1 execution time: {:.2?}", elapsed);
        Ok((input.pop().unwrap().inspections * input.pop().unwrap().inspections).to_string())
    }

    fn part2(mut input: Vec<Monkey>) -> Result<String> {
        let now = Instant::now();
        let handler = input.iter().fold(1u64, |accum, item| accum * item.test);
        for _ in 0..10000 {
            for monkey_i in 0..input.len() {
                while let Some((item, monkey)) = input[monkey_i].test(Some(handler)) {
                    input[monkey].add_item(item);
                }
            }
        }
        input.sort_by(|a, b| a.inspections.cmp(&b.inspections));
        let elapsed = now.elapsed();
        println!("Part 2 execution time: {:.2?}", elapsed);
        Ok((input.pop().unwrap().inspections * input.pop().unwrap().inspections).to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn part1_test() -> Result<()> {
        let parsed_input = Day11::parse(INPUT)?;
        let result = Day11::part1(parsed_input)?;
        assert_eq!(&result, "10605");
        Ok(())
    }

    #[test]
    fn part2_test() -> Result<()> {
        let parsed_input = Day11::parse(INPUT)?;
        let result = Day11::part2(parsed_input)?;
        assert_eq!(&result, "2713310158");
        Ok(())
    }

    const INPUT: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";
}
