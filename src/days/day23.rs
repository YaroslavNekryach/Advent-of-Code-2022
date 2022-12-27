use crate::utils::Day;
use crate::utils::Result;

use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::io::stdout;

pub struct Day23 {}

pub type Pos = (i64, i64);

impl Day23 {
    const DAY: u8 = 23;

    pub fn get_new_pos(elves: &HashSet<Pos>, pos: &Pos, directions: &[[Pos; 3]; 4]) -> Pos {
        let mut empty = 0;
        for dir in [
            (-1, 0),
            (-1, 1),
            (-1, -1),
            (1, 0),
            (1, -1),
            (1, 1),
            (0, -1),
            (0, 1),
        ] {
            let new_pos = (pos.0 + dir.0, pos.1 + dir.1);
            if !elves.contains(&new_pos) {
                empty += 1;
            }
        }
        if empty == 8 {
            return *pos;
        }

        for dirs in directions {
            let mut check_count = 0;
            let mut elve_count = 0;
            for dir in dirs {
                let new_pos = (pos.0 + dir.0, pos.1 + dir.1);

                check_count += 1;
                if elves.contains(&new_pos) {
                    elve_count += 1;
                }
            }
            if check_count > 0 && elve_count == 0 {
                return (pos.0 + dirs[1].0, pos.1 + dirs[1].1);
            }
        }
        *pos
    }
}

impl Day<HashSet<Pos>> for Day23 {
    fn day() -> u8 {
        Self::DAY
    }

    fn parse(input: &str) -> Result<HashSet<Pos>> {
        let mut result: HashSet<Pos> = HashSet::new();
        input.lines().enumerate().for_each(|(y, line)| {
            line.chars().enumerate().for_each(|(x, c)| {
                if c == '#' {
                    result.insert((x as i64, y as i64));
                }
            })
        });
        Ok(result)
    }

    fn part1(mut elves: HashSet<Pos>) -> Result<String> {
        let directions = [
            [(-1, -1), (0, -1), (1, -1)],
            [(-1, 1), (0, 1), (1, 1)],
            [(-1, -1), (-1, 0), (-1, 1)],
            [(1, -1), (1, 0), (1, 1)],
        ];
        let mut di = directions.iter().cycle();
        for _ in 0..10 {
            let mut moves: HashMap<Pos, i64> = HashMap::new();
            let dirs = [
                *di.next().unwrap(),
                *di.next().unwrap(),
                *di.next().unwrap(),
                *di.next().unwrap(),
            ];
            di.next();
            for elf in &elves {
                let new_pos = Day23::get_new_pos(&elves, elf, &dirs);
                *moves.entry(new_pos).or_insert(0) += 1;
            }
            let mut new_elves: HashSet<Pos> = HashSet::new();
            for elf in &elves {
                let new_pos = Day23::get_new_pos(&elves, elf, &dirs);
                if *moves.get(&new_pos).unwrap() > 1 {
                    new_elves.insert(*elf);
                } else {
                    new_elves.insert(new_pos);
                }
            }
            elves = new_elves;
        }
        let x_min = elves.iter().map(|v| v.0).min().unwrap();
        let x_max = elves.iter().map(|v| v.0).max().unwrap();
        let y_min = elves.iter().map(|v| v.1).min().unwrap();
        let y_max = elves.iter().map(|v| v.1).max().unwrap();

        Ok(((x_max - x_min + 1) * (y_max - y_min + 1) - elves.len() as i64).to_string())
    }

    fn part2(mut elves: HashSet<Pos>) -> Result<String> {
        let directions = [
            [(-1, -1), (0, -1), (1, -1)],
            [(-1, 1), (0, 1), (1, 1)],
            [(-1, -1), (-1, 0), (-1, 1)],
            [(1, -1), (1, 0), (1, 1)],
        ];
        let mut di = directions.iter().cycle();
        let mut round = 0;
        loop {
            round += 1;
            let mut moves: HashMap<Pos, i64> = HashMap::new();
            let dirs = [
                *di.next().unwrap(),
                *di.next().unwrap(),
                *di.next().unwrap(),
                *di.next().unwrap(),
            ];
            di.next();
            for elf in &elves {
                let new_pos = Day23::get_new_pos(&elves, elf, &dirs);
                *moves.entry(new_pos).or_insert(0) += 1;
            }
            let mut new_elves: HashSet<Pos> = HashSet::new();
            let mut moved = false;
            for elf in &elves {
                let mut new_pos = Day23::get_new_pos(&elves, elf, &dirs);
                if *moves.get(&new_pos).unwrap() > 1 {
                    new_pos = *elf;
                }
                new_elves.insert(new_pos);
                if new_pos != *elf {
                    moved = true;
                }
            }
            elves = new_elves;
            if !moved {
                return Ok(round.to_string());
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn part1_test() -> Result<()> {
        let parsed_input = Day23::parse(INPUT)?;
        let result = Day23::part1(parsed_input)?;
        assert_eq!(&result, "110");
        Ok(())
    }

    #[test]
    fn part2_test() -> Result<()> {
        let parsed_input = Day23::parse(INPUT)?;
        let result = Day23::part2(parsed_input)?;
        assert_eq!(&result, "20");
        Ok(())
    }

    const INPUT: &str = "..............
..............
.......#......
.....###.#....
...#...#.#....
....#...##....
...#.###......
...##.#.##....
....#..#......
..............
..............
..............";
}
