use crate::utils::{Day, Result};
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::ops::Index;

pub struct Day16 {}

impl Day16 {
    const DAY: u8 = 16;

    pub fn find_max(
        input: &HashMap<String, Valve>,
        current: &str,
        opened: &HashSet<&str>,
        time: u64,
    ) -> u64 {
        if time == 0 {
            return 0;
        }
        let mut results: Vec<u64> = Vec::new();
        let valve = input.get(current).unwrap();
        if !opened.contains(&current) {
            let mut opened_clone = opened.clone();
            opened_clone.insert(current);
            let max = Self::find_max(input, current, &opened_clone, time - 1);
            results.push(max + valve.rate * time);
        }
        for next in &valve.valves {
            results.push(Self::find_max(
                input,
                &next.0,
                opened,
                time.saturating_sub(next.1 as u64),
            ));
        }
        let result = results.iter().max().unwrap_or(&0u64);
        return *result;
    }

    pub fn find_max_with_help(
        input: &HashMap<String, Valve>,
        current: &str,
        el_current: &str,
        opened: &HashSet<&str>,
        time: u64,
        prev: Option<&str>,
        el_prev: Option<&str>,
    ) -> u64 {
        0
    }
}

#[derive(Clone, Debug)]
pub struct Valve {
    pub rate: u64,
    pub valves: Vec<(String, u8)>,
}

impl Day<HashMap<String, Valve>> for Day16 {
    fn day() -> u8 {
        Self::DAY
    }

    fn parse(input: &str) -> Result<HashMap<String, Valve>> {
        let mut result = HashMap::new();
        let re = Regex::new(r"^Valve (..) has flow rate=(\d+); tunnels? leads? to valves? (.*)$")
            .unwrap();
        input.lines().for_each(|line| {
            let captures = re.captures(line).unwrap();
            let valves = captures
                .index(3)
                .split_terminator(", ")
                .map(|v| (v.into(), 1))
                .collect();
            result.insert(
                captures.index(1).to_string(),
                Valve {
                    rate: captures.index(2).parse().unwrap(),
                    valves,
                },
            );
        });
        Ok(result)
    }

    fn part1(mut input: HashMap<String, Valve>) -> Result<String> {
        input
            .clone()
            .into_iter()
            .filter(|v| v.1.rate == 0 && v.1.valves.len() == 2)
            .for_each(|v| {
                println!("Process {}", v.0);
                let v1 = input.get(&v.0).unwrap().valves[0].clone();
                let v2 = input.get(&v.0).unwrap().valves[1].clone();
                let l1 = input
                    .get_mut(&v1.0)
                    .unwrap()
                    .valves
                    .iter_mut()
                    .find(|k| k.0 == v.0)
                    .unwrap();
                println!("from {:?}", v1);

                l1.0 = v2.0.clone();
                l1.1 += v2.1;
                let l2 = input
                    .get_mut(&v2.0)
                    .unwrap()
                    .valves
                    .iter_mut()
                    .find(|k| k.0 == v.0)
                    .unwrap();
                println!("to {:?}", v2);
                l2.0 = v1.0;
                l2.1 += v1.1;
                input.remove(&v.0);
            });
        println!("{:#?}", input);
        // Ok("0".to_string())
        Ok(Day16::find_max(&input, "AA", &HashSet::new(), 29).to_string())
    }

    fn part2(input: HashMap<String, Valve>) -> Result<String> {
        Ok(
            Day16::find_max_with_help(&input, "AA", "AA", &HashSet::new(), 25, None, None)
                .to_string(),
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn part1_test() -> Result<()> {
        let parsed_input = Day16::parse(INPUT)?;
        let result = Day16::part1(parsed_input)?;
        assert_eq!(&result, "1651");
        Ok(())
    }

    #[test]
    fn part2_test() -> Result<()> {
        let parsed_input = Day16::parse(INPUT)?;
        let result = Day16::part2(parsed_input)?;
        assert_eq!(&result, "1707");
        Ok(())
    }

    const INPUT: &str = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";
}
