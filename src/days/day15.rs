use crate::utils::{Day, Pos, Result};
use regex::Regex;
use std::ops::Index;
use std::time::Instant;

pub struct Day15 {}

impl Day15 {
    const DAY: u8 = 15;
}

#[derive(Clone)]
pub struct Sensor {
    s: Pos<i64>,
    b: Pos<i64>,
}

impl Sensor {
    pub fn new(s: Pos<i64>, b: Pos<i64>) -> Self {
        Self { s, b }
    }

    pub fn get_r(&self) -> i64 {
        (self.s.x - self.b.x).abs() + (self.s.y - self.b.y).abs()
    }

    pub fn get_line_range(&self, line: i64) -> Option<(i64, i64)> {
        let r = self.get_r();
        let line_dist = (self.s.y - line).abs();
        if line_dist > r {
            return None;
        }
        let line_range = r - line_dist;
        Some((self.s.x - line_range, self.s.x + line_range))
    }
}

impl Day<Vec<Sensor>> for Day15 {
    fn day() -> u8 {
        Self::DAY
    }

    fn parse(input: &str) -> Result<Vec<Sensor>> {
        Ok(input
            .lines()
            .map(|line| {
                let re = Regex::new(
                    r"^Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)$",
                )
                .unwrap();
                let captures = re.captures(line).unwrap();
                Sensor::new(
                    Pos::<i64>::new(
                        captures.index(1).parse().unwrap(),
                        captures.index(2).parse().unwrap(),
                    ),
                    Pos::<i64>::new(
                        captures.index(3).parse().unwrap(),
                        captures.index(4).parse().unwrap(),
                    ),
                )
            })
            .collect())
    }

    fn part1(input: Vec<Sensor>) -> Result<String> {
        let line: i64 = 2000000;
        // let line: i64 = 10;
        let mut ranges: Vec<(i64, i64)> = input
            .iter()
            .filter_map(|sensor| sensor.get_line_range(line))
            .collect();

        ranges.sort_by(|a, b| a.0.cmp(&b.0));
        let result = ranges
            .iter()
            .fold(Vec::<(i64, i64)>::new(), |mut result, item| {
                match result.last_mut() {
                    None => result.push(*item),
                    Some(last) => {
                        if last.1 < item.0 {
                            result.push(*item);
                        } else {
                            last.1 = item.1.max(last.1);
                        }
                    }
                };
                result
            })
            .iter()
            .map(|(from, to)| to - from) //TODO there is an error - need to include "to" but remove beacon on the line
            .sum::<i64>();
        Ok(result.to_string())
    }

    fn part2(input: Vec<Sensor>) -> Result<String> {
        for line in (0..4000000).rev() {
            let mut ranges: Vec<(i64, i64)> = input
                .iter()
                .filter_map(|sensor| sensor.get_line_range(line))
                .collect();

            ranges.sort_by(|a, b| a.0.cmp(&b.0));
            let result = ranges
                .iter()
                .fold(Vec::<(i64, i64)>::new(), |mut result, item| {
                    match result.last_mut() {
                        None => result.push(*item),
                        Some(last) => {
                            if last.1 + 1 < item.0 {
                                result.push(*item);
                            } else {
                                last.1 = item.1.max(last.1);
                            }
                        }
                    };
                    result
                });
            if result.len() > 1 {
                return Ok(((result[0].1 + 1) * 4000000 + line).to_string());
            }
        }

        Ok("0".to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn part1_test() -> Result<()> {
        let parsed_input = Day15::parse(INPUT)?;
        let result = Day15::part1(parsed_input)?;
        assert_eq!(&result, "26");
        Ok(())
    }

    #[test]
    fn part2_test() -> Result<()> {
        let parsed_input = Day15::parse(INPUT)?;
        let result = Day15::part2(parsed_input)?;
        assert_eq!(&result, "56000011");
        Ok(())
    }

    const INPUT: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3
";
}
