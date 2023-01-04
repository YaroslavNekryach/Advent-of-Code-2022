use crate::utils::{Day, Result};
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::ops::Index;

pub struct Day16 {}

impl Day16 {
    const DAY: u8 = 16;

    fn best_path1(map: &Map, time: u64, pos: String, visited: HashSet<String>) -> u64 {
        let next: Vec<String> = map
            .values
            .keys()
            .cloned()
            .filter(|key| !visited.contains(key))
            .collect();
        let mut results: Vec<u64> = Vec::new();
        for step in next {
            let move_time = *map.graph.get(&pos).unwrap().get(&step).unwrap();
            let time_left = time.checked_sub(move_time + 1);
            if let Some(time_left) = time_left {
                let mut new_visited = visited.clone();
                new_visited.insert(step.clone());
                let res = map.values.get(&step).unwrap() * time_left
                    + Self::best_path1(map, time_left, step, new_visited);
                results.push(res);
            }
        }
        *results.iter().max().unwrap_or(&0)
    }
}

#[derive(Clone, Debug)]
struct Map {
    graph: HashMap<String, HashMap<String, u64>>,
    values: HashMap<String, u64>,
}

impl Map {
    pub fn new(input: HashMap<String, Valve>) -> Self {
        let mut graph = HashMap::new();
        let mut values = HashMap::new();

        for (valve, value) in input {
            if value.rate > 0 {
                values.insert(valve.clone(), value.rate);
            }
            let mut inner_hash = HashMap::new();
            for v in value.valves {
                inner_hash.insert(v, 1);
            }
            graph.insert(valve, inner_hash);
        }

        let mut map = Self { graph, values };
        map.fill();
        map.filter();
        map
    }

    pub fn fill(&mut self) {
        let c = self.clone();
        for (from, to) in &mut self.graph {
            let mut queue: Vec<(String, u64)> =
                to.clone().into_iter().filter(|v| v.1 == 1).collect();
            while !queue.is_empty() {
                let q = queue.remove(0);
                for t in c.graph.get(&q.0).unwrap() {
                    if from != t.0 && !to.contains_key(t.0) {
                        to.insert(t.0.clone(), q.1 + 1);
                        queue.push((t.0.clone(), q.1 + 1))
                    }
                }
            }
        }
    }

    pub fn filter(&mut self) {
        let values = &self.values;
        for (from, to) in &mut self.graph {
            to.retain(|v, _| values.contains_key(v));
        }
        self.graph
            .retain(|v, _| values.contains_key(v) || v == "AA")
    }
}

#[derive(Clone, Debug)]
pub struct Valve {
    pub rate: u64,
    pub valves: Vec<String>,
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
                .map(|v| v.into())
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

    fn part1(input: HashMap<String, Valve>) -> Result<String> {
        let map = Map::new(input);
        let result = Day16::best_path1(&map, 30, "AA".to_string(), HashSet::new());
        Ok(result.to_string())
    }

    fn part2(input: HashMap<String, Valve>) -> Result<String> {
        Ok("0".to_string())
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
