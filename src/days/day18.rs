use crate::utils::{parse_int, Day, PosComponent, Result};
use std::collections::HashSet;
pub struct Day18 {}

type Pos = (u64, u64, u64);

impl Day18 {
    const DAY: u8 = 18;
}

struct Space {
    surfaces: u64,
    fill_surfaces: u64,
    cubes: HashSet<Pos>,
    gas: HashSet<Pos>,
    max: Pos,
}

impl Space {
    pub fn new() -> Self {
        Self {
            surfaces: 0,
            fill_surfaces: 0,
            cubes: HashSet::new(),
            gas: HashSet::new(),
            max: (0, 0, 0),
        }
    }

    pub fn add_cube(&mut self, pos: Pos) {
        let pos = (pos.x() + 1, pos.y() + 1, pos.z() + 1);
        let mut touch = 0;
        if self.cubes.contains(&(pos.x() - 1, pos.y(), pos.z())) {
            touch += 1
        }
        if self.cubes.contains(&(pos.x() + 1, pos.y(), pos.z())) {
            touch += 1
        }
        if self.cubes.contains(&(pos.x(), pos.y() - 1, pos.z())) {
            touch += 1
        }
        if self.cubes.contains(&(pos.x(), pos.y() + 1, pos.z())) {
            touch += 1
        }
        if self.cubes.contains(&(pos.x(), pos.y(), pos.z() - 1)) {
            touch += 1
        }
        if self.cubes.contains(&(pos.x(), pos.y(), pos.z() + 1)) {
            touch += 1
        }
        self.cubes.insert(pos);
        self.surfaces += 6;
        self.surfaces -= touch * 2;
        self.max.0 = self.max.0.max(pos.0 + 1);
        self.max.1 = self.max.1.max(pos.1 + 1);
        self.max.2 = self.max.2.max(pos.2 + 1);
    }

    pub fn fill(&mut self, pos: Pos) {
        if !self.gas.insert(pos) {
            return;
        }
        if pos.x() != 0 {
            let new_pos = (pos.x() - 1, pos.y(), pos.z());
            if self.cubes.contains(&new_pos) {
                self.fill_surfaces += 1
            } else {
                self.fill(new_pos)
            }
        }
        if pos.x() != self.max.x() {
            let new_pos = (pos.x() + 1, pos.y(), pos.z());
            if self.cubes.contains(&new_pos) {
                self.fill_surfaces += 1
            } else {
                self.fill(new_pos)
            }
        }
        if pos.y() != 0 {
            let new_pos = (pos.x(), pos.y() - 1, pos.z());
            if self.cubes.contains(&new_pos) {
                self.fill_surfaces += 1
            } else {
                self.fill(new_pos)
            }
        }
        if pos.y() != self.max.y() {
            let new_pos = (pos.x(), pos.y() + 1, pos.z());
            if self.cubes.contains(&new_pos) {
                self.fill_surfaces += 1
            } else {
                self.fill(new_pos)
            }
        }

        if pos.z() != 0 {
            let new_pos = (pos.x(), pos.y(), pos.z() - 1);
            if self.cubes.contains(&new_pos) {
                self.fill_surfaces += 1
            } else {
                self.fill(new_pos)
            }
        }
        if pos.z() != self.max.z() {
            let new_pos = (pos.x(), pos.y(), pos.z() + 1);
            if self.cubes.contains(&new_pos) {
                self.fill_surfaces += 1
            } else {
                self.fill(new_pos)
            }
        }
    }
}

impl Day<Vec<Pos>> for Day18 {
    fn day() -> u8 {
        Self::DAY
    }

    fn parse(input: &str) -> Result<Vec<Pos>> {
        Ok(input
            .lines()
            .map(|line| {
                let pos: Vec<u64> = line
                    .split_terminator(',')
                    .map(|v| v.parse::<u64>().unwrap())
                    .collect();
                (pos[0], pos[1], pos[2])
            })
            .collect())
    }

    fn part1(input: Vec<Pos>) -> Result<String> {
        let mut space = Space::new();
        for cube in input {
            space.add_cube(cube);
        }
        Ok(space.surfaces.to_string())
    }

    fn part2(input: Vec<Pos>) -> Result<String> {
        let mut space = Space::new();
        for cube in input {
            space.add_cube(cube);
        }
        space.fill((0, 0, 0));
        Ok(space.fill_surfaces.to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn part1_test() -> Result<()> {
        let parsed_input = Day18::parse(INPUT)?;
        let result = Day18::part1(parsed_input)?;
        assert_eq!(&result, "64");
        Ok(())
    }

    #[test]
    fn part2_test() -> Result<()> {
        let parsed_input = Day18::parse(INPUT)?;
        let result = Day18::part2(parsed_input)?;
        assert_eq!(&result, "58");
        Ok(())
    }

    const INPUT: &str = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";
}
