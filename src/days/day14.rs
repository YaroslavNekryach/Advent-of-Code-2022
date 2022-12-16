use crate::utils::{Day, Result, SplitString};
use std::collections::HashMap;
use std::fmt::{Display, Formatter, Write};

pub struct Day14 {}

impl Day14 {
    const DAY: u8 = 14;
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Pos {
    x: u64,
    y: u64,
}

impl Pos {
    pub fn new(x: u64, y: u64) -> Self {
        Pos { x, y }
    }

    pub fn down(&self) -> Self {
        Pos {
            x: self.x,
            y: self.y + 1,
        }
    }

    pub fn left(&self) -> Self {
        Pos {
            x: self.x - 1,
            y: self.y,
        }
    }

    pub fn right(&self) -> Self {
        Pos {
            x: self.x + 1,
            y: self.y,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum State {
    Rock,
    Send,
    Empty,
}

struct Cave {
    map: HashMap<Pos, State>,
    left: u64,
    right: u64,
    bottom: u64,
}

impl Cave {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
            left: 500,
            right: 500,
            bottom: 0,
        }
    }

    pub fn build(&mut self, input: Vec<Vec<Pos>>) {
        for path in input {
            path.windows(2).for_each(|pair| {
                let p1 = pair[0].clone();
                let mut p2 = pair[1].clone();
                let dx = p1.x.cmp(&p2.x) as i64;
                let dy = p1.y.cmp(&p2.y) as i64;
                self.left = p1.x.min(p2.x).min(self.left);
                self.right = p1.x.max(p2.x).max(self.right);
                self.bottom = p1.y.max(p2.y).max(self.bottom);
                while p1 != p2 {
                    self.map.insert(p2.clone(), State::Rock);
                    p2 = Pos::new(
                        (p2.x as i64 + dx).unsigned_abs(),
                        (p2.y as i64 + dy).unsigned_abs(),
                    );
                }
                self.map.insert(p1, State::Rock);
            })
        }
    }

    pub fn state(&self, pos: &Pos) -> State {
        self.map.get(pos).unwrap_or(&State::Empty).clone()
    }

    pub fn drop_send(&mut self, has_floor: bool) -> bool {
        let mut pos = Pos::new(500, 0);
        loop {
            let down = pos.down();
            if self.state(&down) == State::Empty {
                pos = down
            } else if self.state(&down.left()) == State::Empty {
                pos = down.left()
            } else if self.state(&down.right()) == State::Empty {
                pos = down.right()
            } else {
                self.map.insert(pos.clone(), State::Send);
                return pos != Pos::new(500, 0);
            }
            if pos.y > self.bottom {
                if has_floor {
                    self.map.insert(pos.clone(), State::Send);
                    self.left = self.left.min(pos.x);
                    self.right = self.left.max(pos.x);
                }
                return has_floor;
            }
        }
    }

    pub fn send_count(&self) -> u64 {
        let mut result = 0;
        for y in 0..self.bottom + 2 {
            for x in self.left..self.right + 1 {
                if self.state(&Pos::new(x, y)) == State::Send {
                    result += 1;
                }
            }
        }
        result
    }
}

impl Display for Cave {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.bottom + 2 {
            for x in self.left..self.right + 1 {
                let state = self.state(&Pos::new(x, y));
                let char = match state {
                    State::Empty => '.',
                    State::Rock => '#',
                    State::Send => 'o',
                };
                f.write_char(char)?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

impl Day<Vec<Vec<Pos>>> for Day14 {
    fn day() -> u8 {
        Self::DAY
    }

    fn parse(input: &str) -> Result<Vec<Vec<Pos>>> {
        Ok(input
            .lines()
            .map(|line| {
                line.split_terminator(" -> ")
                    .map(|point| {
                        let (x, y) = point.split_by(",").unwrap();
                        Pos::new(x.parse().unwrap(), y.parse().unwrap())
                    })
                    .collect()
            })
            .collect())
    }

    fn part1(input: Vec<Vec<Pos>>) -> Result<String> {
        let mut cave = Cave::new();
        cave.build(input);
        while cave.drop_send(false) {}
        Ok(cave.send_count().to_string())
    }

    fn part2(input: Vec<Vec<Pos>>) -> Result<String> {
        let mut cave = Cave::new();
        cave.build(input);
        while cave.drop_send(true) {}
        Ok(cave.send_count().to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::time::Instant;
    #[test]
    fn part1_test() -> Result<()> {
        let parsed_input = Day14::parse(INPUT)?;
        let result = Day14::part1(parsed_input)?;
        assert_eq!(&result, "24");
        Ok(())
    }

    #[test]
    fn part2_test() -> Result<()> {
        let parsed_input = Day14::parse(INPUT)?;
        let result = Day14::part2(parsed_input)?;
        assert_eq!(&result, "93");
        Ok(())
    }

    const INPUT: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9
";
}
