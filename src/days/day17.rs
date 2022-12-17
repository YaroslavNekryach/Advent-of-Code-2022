use crate::utils::{Day, Result};
use std::collections::HashSet;
use std::fmt::{Display, Formatter, Write};
use std::iter::Cycle;
use std::slice::Iter;

pub struct Day17 {}

impl Day17 {
    const DAY: u8 = 17;

    pub fn get_rocks() -> Vec<Rock> {
        let rock1 = Rock::from([(0, 0), (1, 0), (2, 0), (3, 0)]);
        let rock2 = Rock::from([(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)]);
        let rock3 = Rock::from([(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)]);
        let rock4 = Rock::from([(0, 0), (0, 1), (0, 2), (0, 3)]);
        let rock5 = Rock::from([(0, 0), (0, 1), (1, 0), (1, 1)]);
        vec![rock1, rock2, rock3, rock4, rock5]
    }
}

type Pos = (u64, u64);
type Rock = Vec<Pos>;

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum Direction {
    Left,
    Right,
}

impl Display for Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_char(if self == &Direction::Left { '<' } else { '>' })
            .unwrap();
        Ok(())
    }
}

struct Chamber {
    height: u64,
    fill: HashSet<Pos>,
}

impl Display for Chamber {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in (0..self.height + 1).rev() {
            if y % 100 == 0 {
                f.write_str(&format!("{} \n", y)).unwrap();
            }
            for x in 0u64..7 {
                f.write_char(if self.fill.contains(&(x, y)) {
                    '#'
                } else {
                    '.'
                })
                .unwrap();
            }
            f.write_char('\n').unwrap();
        }
        Ok(())
    }
}

impl Chamber {
    pub fn new() -> Self {
        Self {
            height: 0,
            fill: HashSet::new(),
        }
    }
    pub fn add_rock(&mut self, rock: &Rock, dir_iter: &mut Cycle<Iter<Direction>>) {
        let mut rock: Rock = rock
            .iter()
            .map(|(x, y)| (x + 2, y + 3 + self.height))
            .collect();
        for direction in dir_iter {
            if *direction == Direction::Right {
                if !rock
                    .iter()
                    .any(|pos| pos.0 == 6 || self.fill.contains(&(pos.0 + 1, pos.1)))
                {
                    rock.iter_mut().for_each(|pos| pos.0 += 1);
                }
            } else if *direction == Direction::Left
                && !rock
                    .iter()
                    .any(|pos| pos.0 == 0 || self.fill.contains(&(pos.0 - 1, pos.1)))
            {
                rock.iter_mut().for_each(|pos| pos.0 -= 1);
            }
            if !rock
                .iter()
                .any(|pos| pos.1 == 0 || self.fill.contains(&(pos.0, pos.1 - 1)))
            {
                rock.iter_mut().for_each(|pos| pos.1 -= 1);
            } else {
                rock.iter().for_each(|pos| {
                    self.fill.insert(*pos);
                });
                self.height = self
                    .height
                    .max(rock.iter().map(|pos| pos.1).max().unwrap() + 1);
                // let l = ((self.height as i64 - 521) % 3216);
                // if l < 2 {
                //     println!("{} {}", self.height, i);
                // }
                return;
            }
        }
    }
}

impl Day<Vec<Direction>> for Day17 {
    fn day() -> u8 {
        Self::DAY
    }

    fn parse(input: &str) -> Result<Vec<Direction>> {
        Ok(input
            .chars()
            .map(|c| match c {
                '<' => Direction::Left,
                '>' => Direction::Right,
                _ => panic!(),
            })
            .collect())
    }

    fn part1(input: Vec<Direction>) -> Result<String> {
        let rocks = Day17::get_rocks();
        let mut rock_iter = rocks.iter().cycle();
        let mut direction_iter = input.iter().cycle();
        let mut chamber = Chamber::new();
        for _ in 0..2022 {
            let next_rock = rock_iter.next().unwrap();
            chamber.add_rock(next_rock, &mut direction_iter);
        }
        // println!("{}", chamber);
        Ok(chamber.height.to_string())
    }

    fn part2(input: Vec<Direction>) -> Result<String> {
        let rocks = Day17::get_rocks();
        let mut rock_iter = rocks.iter().cycle();
        let mut direction_iter = input.iter().cycle();
        let mut chamber = Chamber::new();
        // 355 2090: +1735
        // 536 3231: +2695
        // 576368875
        // (1000000000000 - 355) / 1735
        for _ in 0..1875 {
            let next_rock = rock_iter.next().unwrap();
            chamber.add_rock(next_rock, &mut direction_iter);
        }
        Ok((chamber.height + 2695 * 576368875).to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn part1_test() -> Result<()> {
        let parsed_input = Day17::parse(INPUT)?;
        let result = Day17::part1(parsed_input)?;
        assert_eq!(&result, "3068");
        Ok(())
    }

    #[test]
    fn part2_test() -> Result<()> {
        let parsed_input = Day17::parse(INPUT)?;
        let result = Day17::part2(parsed_input)?;
        assert_eq!(&result, "1514285714288");
        Ok(())
    }

    const INPUT: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
}
