use crate::days::day10::Instruction::{Addx, Noop};
use crate::utils::{Day, Result};

pub struct Day10 {}

impl Day10 {
    const DAY: u8 = 10;
}

#[derive(Clone)]
pub enum Instruction {
    Noop,
    Addx(i64),
}

impl Instruction {
    fn parse(s: &str) -> Vec<Self> {
        if s == "noop" {
            vec![Noop]
        } else {
            let (_, value) = s.split_at(5);
            vec![Noop, Addx(value.parse().unwrap())]
        }
    }

    fn _to_string(&self) -> String {
        match self {
            Noop => "noop".to_string(),
            Addx(v) => format!("addx {}", v),
        }
    }
}

impl Day<Vec<Instruction>> for Day10 {
    fn day() -> u8 {
        Self::DAY
    }

    fn parse(input: &str) -> Result<Vec<Instruction>> {
        Ok(input.lines().flat_map(Instruction::parse).collect())
    }

    fn part1(input: Vec<Instruction>) -> Result<String> {
        let mut x = 1i64;
        let mut result: i64 = 0;
        let cycle_checkpoints = [20, 60, 100, 140, 180, 220];
        for (cycle, instruction) in input.into_iter().enumerate() {
            for checkpoint in cycle_checkpoints {
                if cycle == checkpoint - 1 {
                    result += x * checkpoint as i64;
                }
            }
            if let Addx(value) = instruction {
                x += value
            }
        }
        Ok(result.to_string())
    }

    fn part2(input: Vec<Instruction>) -> Result<String> {
        let mut x = 1i64;
        let mut result = "\n".to_string();
        let mut row = ".".repeat(40);
        for (cycle, instruction) in input.into_iter().enumerate() {
            if cycle > 0 && (cycle) % 40 == 0 {
                result += &format!("{}\n", row);
                row = ".".repeat(40);
            }

            let pos = cycle as i64 % 40;
            if (pos - x).abs() <= 1 {
                row.replace_range(pos as usize..pos as usize + 1, "#");
            }

            if let Addx(value) = instruction {
                x += value
            }
        }
        result += &row;
        Ok(result.to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn part1_test() -> Result<()> {
        let parsed_input = Day10::parse(INPUT)?;
        let result = Day10::part1(parsed_input)?;
        assert_eq!(&result, "13140");
        Ok(())
    }

    #[test]
    fn part2_test() -> Result<()> {
        let parsed_input = Day10::parse(INPUT)?;
        let result = Day10::part2(parsed_input)?;
        assert_eq!(
            &result,
            "
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."
        );
        Ok(())
    }

    const INPUT: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";
}
