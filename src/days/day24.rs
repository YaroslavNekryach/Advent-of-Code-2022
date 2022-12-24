use crate::utils::{Day, PosComponent2D, Result};
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter, Write};

pub struct Day24 {}

pub type Pos = (u64, u64);

impl Day24 {
    const DAY: u8 = 24;
}

#[derive(Clone)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl From<char> for Direction {
    fn from(s: char) -> Self {
        match s {
            '>' => Direction::Right,
            '<' => Direction::Left,
            'v' => Direction::Down,
            '^' => Direction::Up,
            _ => panic!(),
        }
    }
}

#[derive(Clone)]
pub struct Blizzard {
    pos: Pos,
    direction: Direction,
}

impl Blizzard {
    pub fn new(c: char, x: u64, y: u64) -> Self {
        Self {
            direction: c.into(),
            pos: (x, y),
        }
    }

    pub fn step(&mut self, width: u64, height: u64) {
        match self.direction {
            Direction::Up => {
                self.pos.1 = if self.pos.1 == 0 {
                    height - 1
                } else {
                    self.pos.1 - 1
                }
            }
            Direction::Down => {
                self.pos.1 = if self.pos.1 == height - 1 {
                    0
                } else {
                    self.pos.1 + 1
                }
            }
            Direction::Left => {
                self.pos.0 = if self.pos.0 == 0 {
                    width - 1
                } else {
                    self.pos.0 - 1
                }
            }
            Direction::Right => {
                self.pos.0 = if self.pos.0 == width - 1 {
                    0
                } else {
                    self.pos.0 + 1
                }
            }
        }
    }
}

#[derive(Clone)]
pub struct Map {
    blizzards: Vec<Blizzard>,
    elves: HashSet<Pos>,
    width: u64,
    height: u64,
}

impl Map {
    pub fn new(blizzards: Vec<Blizzard>, width: u64, height: u64) -> Self {
        Self {
            blizzards,
            elves: HashSet::new(),
            width,
            height,
        }
    }
    pub fn addElv(&mut self) {
        if !self.blizzards.iter().any(|b| b.pos == (0, 0)) {
            self.elves.insert((0, 0));
        }
    }

    pub fn addElvEnd(&mut self) {
        let pos = (self.width - 1, self.height - 1);
        if !self.blizzards.iter().any(|b| b.pos == pos) {
            self.elves.insert(pos);
        }
    }

    pub fn step(&mut self) {
        self.blizzards
            .iter_mut()
            .for_each(|blizzard| blizzard.step(self.width, self.height));
        let mut blizzard_set: HashSet<Pos> = HashSet::new();
        self.blizzards.iter().for_each(|b| {
            blizzard_set.insert(b.pos);
        });
        let mut elves: HashSet<Pos> = HashSet::new();

        self.elves.iter().for_each(|elf| {
            if elf.x() > 0 {
                let pos = (elf.x() - 1, elf.y());
                if !blizzard_set.contains(&pos) {
                    elves.insert(pos);
                }
            }
            if elf.x() < self.width - 1 {
                let pos = (elf.x() + 1, elf.y());
                if !blizzard_set.contains(&pos) {
                    elves.insert(pos);
                }
            }
            if elf.y() > 0 {
                let pos = (elf.x(), elf.y() - 1);
                if !blizzard_set.contains(&pos) {
                    elves.insert(pos);
                }
            }
            if elf.y() < self.height - 1 {
                let pos = (elf.x(), elf.y() + 1);
                if !blizzard_set.contains(&pos) {
                    elves.insert(pos);
                }
            }
            if !blizzard_set.contains(elf) {
                elves.insert(*elf);
            }
        });

        self.elves = elves;
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                if self.elves.contains(&(x, y)) {
                    f.write_char('E').unwrap()
                } else {
                    let bs: Vec<&Blizzard> =
                        self.blizzards.iter().filter(|b| b.pos == (x, y)).collect();
                    if bs.len() == 1 {
                        f.write_char(match bs[0].direction {
                            Direction::Up => '^',
                            Direction::Down => 'v',
                            Direction::Left => '<',
                            Direction::Right => '>',
                        })
                        .unwrap();
                    } else {
                        f.write_char(match bs.len() {
                            0 => '.',
                            2 => '2',
                            3 => '3',
                            4 => '4',
                            _ => panic!(),
                        })
                        .unwrap();
                    }
                }
            }
            f.write_char('\n').unwrap();
        }
        Ok(())
    }
}

impl Day<Map> for Day24 {
    fn day() -> u8 {
        Self::DAY
    }

    fn parse(input: &str) -> Result<Map> {
        let lines: Vec<&str> = input.lines().collect();
        let height = lines.len() as u64 - 2;
        let width = lines[0].len() as u64 - 2;
        let mut blizzards: Vec<Blizzard> = Vec::new();
        lines.iter().enumerate().for_each(|(y, line)| {
            line.chars().enumerate().for_each(|(x, c)| {
                if c != '.' && c != '#' {
                    blizzards.push(Blizzard::new(c, x as u64 - 1, y as u64 - 1))
                }
            })
        });
        Ok(Map::new(blizzards, width, height))
    }

    fn part1(mut map: Map) -> Result<String> {
        let mut step = 0;
        map.step();
        step += 1;
        while !map.elves.contains(&(map.width - 1, map.height - 1)) {
            if map.elves.is_empty() {
                map.addElv();
            }
            map.step();
            step += 1;
        }

        Ok((step + 1).to_string())
    }

    fn part2(mut map: Map) -> Result<String> {
        let mut step = 0;
        map.step();
        step += 1;
        while !map.elves.contains(&(map.width - 1, map.height - 1)) {
            if map.elves.is_empty() {
                map.addElv();
            }
            map.step();
            step += 1;
        }
        map.step();
        step += 1;
        map.elves = HashSet::new();
        while !map.elves.contains(&(0, 0)) {
            if map.elves.is_empty() {
                map.addElvEnd();
            }
            map.step();
            step += 1;
        }
        map.step();
        step += 1;
        map.elves = HashSet::new();
        while !map.elves.contains(&(map.width - 1, map.height - 1)) {
            if map.elves.is_empty() {
                map.addElv();
            }
            map.step();
            step += 1;
        }

        Ok((step + 1).to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn part1_test() -> Result<()> {
        let parsed_input = Day24::parse(INPUT)?;
        let result = Day24::part1(parsed_input)?;
        assert_eq!(&result, "18");
        Ok(())
    }

    #[test]
    fn part2_test() -> Result<()> {
        let parsed_input = Day24::parse(INPUT)?;
        let result = Day24::part2(parsed_input)?;
        assert_eq!(&result, "54");
        Ok(())
    }

    const INPUT: &str = "#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#";
}
