use crate::utils::{Day, Result};
use regex::Regex;
use std::ops::Index;

pub struct Day19 {}

impl Day19 {
    const DAY: u8 = 19;
}

#[derive(Clone, Debug)]
pub struct Blueprint {
    index: u64,
    ore: u64,             // ore
    clay: u64,            // ore
    obsidian: (u64, u64), // ore, clay
    geode: (u64, u64),    // ore, obsidian
}

#[derive(Clone, Copy, Debug)]
struct State<'a> {
    blueprint: &'a Blueprint,
    ore: u64,
    clay: u64,
    obsidian: u64,
    geode: u64,
    ore_robots: u64,
    clay_robots: u64,
    obsidian_robots: u64,
    geode_robots: u64,
    minute: u64,
}

impl<'a> State<'a> {
    pub fn new(blueprint: &'a Blueprint) -> Self {
        Self {
            blueprint,
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,
            ore_robots: 1,
            clay_robots: 0,
            obsidian_robots: 0,
            geode_robots: 0,
            minute: 0,
        }
    }

    pub fn buy_ore(&mut self) -> Option<Self> {
        if (self.ore - self.ore_robots)
            .checked_sub(self.blueprint.ore)
            .is_some()
        {
            self.ore -= self.blueprint.ore;
            self.ore_robots += 1;
            Some(self.clone())
        } else {
            None
        }
    }

    pub fn buy_clay(&mut self) -> Option<Self> {
        if (self.ore - self.ore_robots)
            .checked_sub(self.blueprint.clay)
            .is_some()
        {
            self.ore -= self.blueprint.clay;
            self.clay_robots += 1;
            Some(self.clone())
        } else {
            None
        }
    }

    pub fn buy_obsidian(&mut self) -> Option<Self> {
        if (self.ore - self.ore_robots)
            .checked_sub(self.blueprint.obsidian.0)
            .is_some()
            && (self.clay - self.clay_robots)
                .checked_sub(self.blueprint.obsidian.1)
                .is_some()
        {
            self.ore -= self.blueprint.obsidian.0;
            self.clay -= self.blueprint.obsidian.1;
            self.obsidian_robots += 1;
            Some(self.clone())
        } else {
            None
        }
    }

    pub fn buy_geode(&mut self) -> Option<Self> {
        if (self.ore - self.ore_robots)
            .checked_sub(self.blueprint.geode.0)
            .is_some()
            && (self.obsidian - self.obsidian_robots)
                .checked_sub(self.blueprint.geode.1)
                .is_some()
        {
            self.ore -= self.blueprint.geode.0;
            self.obsidian -= self.blueprint.geode.1;
            self.geode_robots += 1;
            Some(self.clone())
        } else {
            None
        }
    }

    pub fn best(&mut self) -> Self {
        if self.minute >= 32 {
            return self.clone();
        }
        self.minute += 1;
        self.ore += self.ore_robots;
        self.clay += self.clay_robots;
        self.obsidian += self.obsidian_robots;
        self.geode += self.geode_robots;
        let mut best_states: Vec<State> = vec![];
        let mut stay = 0;

        if let Some(state) = self.clone().buy_geode() {
            best_states.push(state.clone().best());
            stay += 1;
        } else {
            if let Some(state) = self.clone().buy_obsidian() {
                best_states.push(state.clone().best());
                stay += 1;
            } else {
                if let Some(state) = self.clone().buy_clay() {
                    best_states.push(state.clone().best());
                    stay += 1;
                }
                if let Some(state) = self.clone().buy_ore() {
                    best_states.push(state.clone().best());
                    stay += 1;
                }
            }
        }
        if stay < 2 {
            best_states.push(self.clone().best());
        }

        *best_states
            .iter()
            .max_by(|a, b| a.geode.cmp(&b.geode))
            .unwrap()
    }
}

impl Day<Vec<Blueprint>> for Day19 {
    fn day() -> u8 {
        Self::DAY
    }

    fn parse(input: &str) -> Result<Vec<Blueprint>> {
        Ok(input.lines().map(|line| {
            let re = Regex::new(r"Blueprint (\d+): Each ore robot costs (\d+) ore\. Each clay robot costs (\d+) ore\. Each obsidian robot costs (\d+) ore and (\d+) clay\. Each geode robot costs (\d+) ore and (\d+) obsidian\.").unwrap();
            let captures = re.captures(line).unwrap();
            Blueprint {
                index: captures.index(1).parse().unwrap(),
                ore: captures.index(2).parse().unwrap(),
                clay: captures.index(3).parse().unwrap(),
                obsidian: (captures.index(4).parse().unwrap(), captures.index(5).parse().unwrap()),
                geode: (captures.index(6).parse().unwrap(), captures.index(7).parse().unwrap()),
            }

        })
            .collect())
    }

    fn part1(input: Vec<Blueprint>) -> Result<String> {
        return Ok("0".to_string());
        Ok(input
            .iter()
            .map(|bp| {
                println!("{}", bp.index);
                State::new(bp).best().geode * bp.index
            })
            .sum::<u64>()
            .to_string())
    }

    fn part2(input: Vec<Blueprint>) -> Result<String> {
        Ok(input
            .iter()
            .take(3)
            .map(|bp| {
                println!("{}", bp.index);
                State::new(bp).best().geode
            })
            .reduce(|a, b| a * b)
            .unwrap()
            .to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn part1_test() -> Result<()> {
        let parsed_input = Day19::parse(INPUT)?;
        let result = Day19::part1(parsed_input)?;
        assert_eq!(&result, "33");
        Ok(())
    }

    #[test]
    fn part2_test() -> Result<()> {
        let parsed_input = Day19::parse(INPUT)?;
        let result = Day19::part2(parsed_input)?;
        assert_eq!(&result, "45000");
        Ok(())
    }

    const INPUT: &str = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
    Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";
}
