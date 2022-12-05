use crate::utils::{Day, Result, SplitString};
use regex::Regex;
use std::collections::HashMap;
use std::ops::Index;

pub struct Day5 {}

impl Day5 {
    const DAY: u8 = 5;
}

type Crate = char;

type Input = (Stacks, Vec<Procedure>);

#[derive(Clone, Debug)]
pub struct Stacks {
    stack: HashMap<u8, Vec<Crate>>,
}

impl Stacks {
    pub fn parse(input: &str) -> Result<Self> {
        let mut stacks = Stacks::init();
        input
            .lines()
            .last()
            .ok_or("no input")?
            .split_whitespace()
            .map(|v| v.parse::<u8>().unwrap())
            .for_each(|index| stacks.init_stack(index));
        input.lines().rev().skip(1).for_each(|line| {
            line.chars().enumerate().for_each(|(index, value)| {
                if value.is_alphabetic() {
                    stacks.put((index as u8 - 1) / 4 + 1, value).unwrap();
                }
            })
        });
        Ok(stacks)
    }

    fn init() -> Self {
        Self {
            stack: HashMap::new(),
        }
    }

    fn init_stack(&mut self, index: u8) {
        self.stack.insert(index, Vec::new());
    }

    fn put(&mut self, index: u8, item: Crate) -> Result<()> {
        self.stack
            .get_mut(&index)
            .ok_or("Stack does not exist")?
            .push(item);
        Ok(())
    }

    fn get(&mut self, index: u8) -> Result<Crate> {
        Ok(self
            .stack
            .get_mut(&index)
            .ok_or("Stack does not exist")?
            .pop()
            .ok_or("Stack is empty")?)
    }

    fn move_crate(&mut self, from: u8, to: u8) -> Result<()> {
        let item = self.get(from)?;
        self.put(to, item)?;
        Ok(())
    }

    pub fn exec_part1(&mut self, procedure: &Procedure) -> Result<()> {
        for _ in 0..procedure.count {
            self.move_crate(procedure.from, procedure.to)?;
        }

        Ok(())
    }

    pub fn exec_part2(&mut self, procedure: &Procedure) -> Result<()> {
        let crates: Vec<Crate> = (0..procedure.count)
            .map(|_| self.get(procedure.from).unwrap())
            .collect();
        crates
            .iter()
            .rev()
            .for_each(|item| self.put(procedure.to, *item).unwrap());
        Ok(())
    }

    pub fn read_top_crates(&self) -> Result<String> {
        let mut result: Vec<Crate> = Vec::new();
        for i in 1..self.stack.len() + 1 {
            let c = self.stack.get(&(i as u8)).unwrap().last().unwrap();
            result.push(*c);
        }
        Ok(result.iter().collect())
    }
}

#[derive(Clone, Debug)]
pub struct Procedure {
    count: u8,
    from: u8,
    to: u8,
}

impl Procedure {
    pub fn parse(input: &str) -> Result<Self> {
        let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
        let captures = re.captures(input).ok_or("Invalid input")?;
        Ok(Self {
            count: captures.index(1).parse()?,
            from: captures.index(2).parse()?,
            to: captures.index(3).parse()?,
        })
    }
}

impl Day<Input> for Day5 {
    fn day() -> u8 {
        Self::DAY
    }

    fn parse(input: &str) -> Result<Input> {
        let (stacks_str, procedure_str) = input.split_by("\n\n")?;
        let stacks = Stacks::parse(stacks_str)?;
        let procedure = procedure_str
            .lines()
            .map(Procedure::parse)
            .map(|v| v.unwrap())
            .collect();
        Ok((stacks, procedure))
    }

    fn part1((mut stacks, procedures): Input) -> Result<String> {
        procedures
            .iter()
            .for_each(|procedure| stacks.exec_part1(procedure).unwrap());
        stacks.read_top_crates()
    }

    fn part2((mut stacks, procedures): Input) -> Result<String> {
        procedures
            .iter()
            .for_each(|procedure| stacks.exec_part2(procedure).unwrap());
        stacks.read_top_crates()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn part1_test() -> Result<()> {
        let parsed_input = Day5::parse(INPUT)?;
        let result = Day5::part1(parsed_input)?;
        assert_eq!(&result, "CMZ");
        Ok(())
    }

    #[test]
    fn part2_test() -> Result<()> {
        let parsed_input = Day5::parse(INPUT)?;
        let result = Day5::part2(parsed_input)?;
        assert_eq!(&result, "MCD");
        Ok(())
    }

    const INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
}
