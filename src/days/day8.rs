use crate::utils::{parse_int, Day, Result};
use std::collections::HashSet;
use std::fmt::format;

pub struct Day8 {}

impl Day8 {
    const DAY: u8 = 8;
}

impl Day<Vec<Vec<i8>>> for Day8 {
    fn day() -> u8 {
        Self::DAY
    }

    fn parse(input: &str) -> Result<Vec<Vec<i8>>> {
        Ok(input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_string().parse::<i8>().unwrap())
                    .collect::<Vec<i8>>()
            })
            .collect())
    }

    fn part1(input: Vec<Vec<i8>>) -> Result<String> {
        let mut set = HashSet::<String>::new();
        for y in 0..input.len() {
            let mut max = -1i8;
            for x in 0..input[y].len() {
                if input[y][x] > max {
                    set.insert(format!("{}_{}", x, y));
                    max = input[y][x]
                };
            }
        }
        for y in 0..input.len() {
            let mut max = -1i8;
            for x in (0..input[y].len()).rev() {
                if input[y][x] > max {
                    set.insert(format!("{}_{}", x, y));
                    max = input[y][x]
                };
            }
        }

        for x in 0..input.len() {
            let mut max = -1i8;
            for y in 0..input[x].len() {
                if input[y][x] > max {
                    set.insert(format!("{}_{}", x, y));
                    max = input[y][x]
                };
            }
        }

        for x in 0..input.len() {
            let mut max = -1i8;
            for y in (0..input[x].len()).rev() {
                if input[y][x] > max {
                    set.insert(format!("{}_{}", x, y));
                    max = input[y][x]
                };
            }
        }
        Ok(set.len().to_string())
    }

    fn part2(input: Vec<Vec<i8>>) -> Result<String> {
        let mut res = 0;
        for y in 1..input.len() - 1 {
            for x in 1..input[y].len() - 1 {
                let mut mid_res = 1;
                let mut count = 0;
                for d in (0..x).rev() {
                    count += 1;
                    if input[y][d] >= input[y][x] {
                        break;
                    }
                }

                mid_res *= count;
                let mut count = 0;
                for d in x + 1..input[y].len() {
                    count += 1;
                    if input[y][d] >= input[y][x] {
                        break;
                    }
                }

                mid_res *= count;
                let mut count = 0;
                for d in y + 1..input.len() {
                    count += 1;
                    if input[d][x] >= input[y][x] {
                        break;
                    }
                }

                mid_res *= count;
                let mut count = 0;
                for d in (0..y).rev() {
                    count += 1;
                    if input[d][x] >= input[y][x] {
                        break;
                    }
                }
                mid_res *= count;
                if mid_res > res {
                    res = mid_res;
                }
            }
        }
        Ok(res.to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn part1_test() -> Result<()> {
        let parsed_input = Day8::parse(INPUT)?;
        let result = Day8::part1(parsed_input)?;
        assert_eq!(&result, "21");
        Ok(())
    }

    #[test]
    fn part2_test() -> Result<()> {
        let parsed_input = Day8::parse(INPUT)?;
        let result = Day8::part2(parsed_input)?;
        assert_eq!(&result, "8");
        Ok(())
    }

    const INPUT: &str = "30373
25512
65332
33549
35390";
}
