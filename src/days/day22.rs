use crate::utils::{parse_int, Day, Result, SplitString};
use regex::Regex;
use serde_json::to_string;
use std::fmt::{format, Display, Formatter, Write};
use std::ops::Index;
use std::vec;

pub struct Day22 {}

impl Day22 {
    const DAY: u8 = 22;
    const FACE_SIZE: usize = 50;

    fn step(pos: (usize, usize), dir: &Direction) -> (usize, usize) {
        match dir {
            Direction::Right => (pos.0 + 1, pos.1),
            Direction::Left => (pos.0 - 1, pos.1),
            Direction::Up => (pos.0, pos.1 - 1),
            Direction::Down => (pos.0, pos.1 + 1),
        }
    }

    fn is_edge(pos: (usize, usize), dir: &Direction) -> bool {
        match dir {
            Direction::Right => pos.0 == Day22::FACE_SIZE - 1,
            Direction::Left => pos.0 == 0,
            Direction::Up => pos.1 == 0,
            Direction::Down => pos.1 == Day22::FACE_SIZE - 1,
        }
    }

    fn next_face_part1(
        face: usize,
        pos: (usize, usize),
        dir: &Direction,
    ) -> (usize, (usize, usize), Direction) {
        let new_fase = match face {
            0 => match dir {
                Direction::Right => 1,
                Direction::Down => 2,
                Direction::Left => 1,
                Direction::Up => 4,
            },
            1 => match dir {
                Direction::Right => 0,
                Direction::Down => 1,
                Direction::Left => 0,
                Direction::Up => 1,
            },
            2 => match dir {
                Direction::Right => 2,
                Direction::Down => 4,
                Direction::Left => 2,
                Direction::Up => 0,
            },
            3 => match dir {
                Direction::Right => 4,
                Direction::Down => 5,
                Direction::Left => 4,
                Direction::Up => 5,
            },
            4 => match dir {
                Direction::Right => 3,
                Direction::Down => 0,
                Direction::Left => 3,
                Direction::Up => 2,
            },
            5 => match dir {
                Direction::Right => 5,
                Direction::Down => 3,
                Direction::Left => 5,
                Direction::Up => 3,
            },
            _ => panic!("error"),
        };
        let new_pos = match dir {
            Direction::Right => (0, pos.1),
            Direction::Down => (pos.0, 0),
            Direction::Left => (Day22::FACE_SIZE - 1, pos.1),
            Direction::Up => (pos.0, Day22::FACE_SIZE - 1),
        };
        (new_fase, new_pos, *dir)
    }

    fn next_face_part2(
        face: usize,
        pos: (usize, usize),
        dir: &Direction,
    ) -> (usize, (usize, usize), Direction) {
        let ms = Day22::FACE_SIZE - 1;
        match face {
            0 => match dir {
                Direction::Right => (1, (0, pos.1), Direction::Right),
                Direction::Down => (2, (pos.0, 0), Direction::Down),
                Direction::Left => (3, (0, ms - pos.1), Direction::Right),
                Direction::Up => (5, (0, pos.0), Direction::Right),
            },
            1 => match dir {
                Direction::Right => (4, (ms, ms - pos.1), Direction::Left),
                Direction::Down => (2, (ms, pos.0), Direction::Left),
                Direction::Left => (0, (ms, pos.1), Direction::Left),
                Direction::Up => (5, (pos.0, ms), Direction::Up),
            },
            2 => match dir {
                Direction::Right => (1, (pos.1, ms), Direction::Up),
                Direction::Down => (4, (pos.0, 0), Direction::Down),
                Direction::Left => (3, (pos.1, 0), Direction::Down),
                Direction::Up => (0, (pos.0, ms), Direction::Up),
            },
            3 => match dir {
                Direction::Right => (4, (0, pos.1), Direction::Right),
                Direction::Down => (5, (pos.0, 0), Direction::Down),
                Direction::Left => (0, (0, ms - pos.1), Direction::Right),
                Direction::Up => (2, (0, pos.0), Direction::Right),
            },
            4 => match dir {
                Direction::Right => (1, (ms, ms - pos.1), Direction::Left),
                Direction::Down => (5, (ms, pos.0), Direction::Left),
                Direction::Left => (3, (ms, pos.1), Direction::Left),
                Direction::Up => (2, (pos.0, ms), Direction::Up),
            },
            5 => match dir {
                Direction::Right => (4, (pos.1, ms), Direction::Up),
                Direction::Down => (1, (pos.0, 0), Direction::Down),
                Direction::Left => (0, (pos.1, 0), Direction::Down),
                Direction::Up => (3, (pos.0, ms), Direction::Up),
            },
            _ => panic!("error"),
        }
    }
}

#[derive(Copy, Clone)]
enum Direction {
    Right = 0,
    Down = 1,
    Left = 2,
    Up = 3,
}
impl Direction {
    pub fn turn(&mut self, c: char) {
        if c == 'R' {
            *self = match self {
                Direction::Left => Direction::Up,
                Direction::Up => Direction::Right,
                Direction::Right => Direction::Down,
                Direction::Down => Direction::Left,
            };
        } else {
            *self = match self {
                Direction::Left => Direction::Down,
                Direction::Up => Direction::Left,
                Direction::Right => Direction::Up,
                Direction::Down => Direction::Right,
            };
        }
    }
}

#[derive(Clone)]
pub struct Face {
    map: [[char; Day22::FACE_SIZE]; Day22::FACE_SIZE],
    offset: (usize, usize),
}

impl Face {
    pub fn new(input: &Vec<&str>, offset: (usize, usize)) -> Self {
        let mut map: [[char; Day22::FACE_SIZE]; Day22::FACE_SIZE] =
            [['.'; Day22::FACE_SIZE]; Day22::FACE_SIZE];

        for y in 0..Day22::FACE_SIZE {
            for x in 0..Day22::FACE_SIZE {
                if input[y + offset.1].chars().nth(x + offset.0).unwrap() == '#' {
                    map[y][x] = '#'
                }
            }
        }
        Self { map, offset }
    }
}

impl Display for Face {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in 0..Day22::FACE_SIZE {
            for x in 0..Day22::FACE_SIZE {
                f.write_char(self.map[y][x]).unwrap();
            }
            f.write_char('\n').unwrap()
        }
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct Instruction {
    step: usize,
    turn: char,
}

impl Day<(Vec<Face>, Vec<Instruction>)> for Day22 {
    fn day() -> u8 {
        Self::DAY
    }

    fn parse(input: &str) -> Result<(Vec<Face>, Vec<Instruction>)> {
        let (map, instructions) = input.split_by("\n\n").unwrap();
        let lines: Vec<&str> = map.lines().collect();
        let re = Regex::new(r"(\d+)([RL])").unwrap();
        let instructions = format!("{}L0R", instructions);
        let captures = re.captures_iter(&instructions);
        let instructions: Vec<Instruction> = captures
            .map(|v| Instruction {
                step: v.index(1).parse::<usize>().unwrap(),
                turn: v.index(2).chars().next().unwrap(),
            })
            .collect();
        let size = Day22::FACE_SIZE;
        Ok((
            vec![
                Face::new(&lines, (size, 0)),
                Face::new(&lines, (size * 2, 0)),
                Face::new(&lines, (size, size)),
                Face::new(&lines, (0, size * 2)),
                Face::new(&lines, (size, size * 2)),
                Face::new(&lines, (0, size * 3)),
            ],
            instructions,
        ))
    }

    fn part1((faces, instructions): (Vec<Face>, Vec<Instruction>)) -> Result<String> {
        let mut face: usize = 0;
        let mut pos = (0, 0);
        let mut direction = Direction::Right;
        for instruction in instructions {
            for _ in 0..instruction.step {
                if Day22::is_edge(pos, &direction) {
                    let (new_face, new_pos, new_dir) =
                        Day22::next_face_part1(face, pos, &direction);
                    if faces[new_face].map[new_pos.1][new_pos.0] == '#' {
                        break;
                    }
                    face = new_face;
                    pos = new_pos;
                    direction = new_dir;
                } else {
                    let new_pos = Day22::step(pos, &direction);
                    if faces[face].map[new_pos.1][new_pos.0] == '#' {
                        break;
                    }
                    pos = new_pos;
                }
            }
            direction.turn(instruction.turn);
        }
        let row = faces[face].offset.1 + pos.1 + 1;
        let col = faces[face].offset.0 + pos.0 + 1;
        let dir = direction as usize;
        Ok((row * 1000 + col * 4 + dir).to_string())
    }

    fn part2((faces, instructions): (Vec<Face>, Vec<Instruction>)) -> Result<String> {
        let mut face: usize = 0;
        let mut pos = (0, 0);
        let mut direction = Direction::Right;
        for instruction in instructions {
            for _ in 0..instruction.step {
                if Day22::is_edge(pos, &direction) {
                    let (new_face, new_pos, new_dir) =
                        Day22::next_face_part2(face, pos, &direction);
                    if faces[new_face].map[new_pos.1][new_pos.0] == '#' {
                        break;
                    }
                    face = new_face;
                    pos = new_pos;
                    direction = new_dir;
                } else {
                    let new_pos = Day22::step(pos, &direction);
                    if faces[face].map[new_pos.1][new_pos.0] == '#' {
                        break;
                    }
                    pos = new_pos;
                }
            }
            direction.turn(instruction.turn);
        }
        let row = faces[face].offset.1 + pos.1 + 1;
        let col = faces[face].offset.0 + pos.0 + 1;
        let dir = direction as usize;
        Ok((row * 1000 + col * 4 + dir).to_string())
    }
}
