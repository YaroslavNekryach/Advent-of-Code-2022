use crate::utils::{parse_int, Day, Result};
use std::ops::Index;

pub struct Day20 {}

impl Day20 {
    const DAY: u8 = 20;
}

impl Day<Vec<i64>> for Day20 {
    fn day() -> u8 {
        Self::DAY
    }

    fn parse(input: &str) -> Result<Vec<i64>> {
        Ok(input.lines().map(|v| v.parse::<i64>().unwrap()).collect())
    }

    fn part1(mut input: Vec<i64>) -> Result<String> {
        let mut enumerated: Vec<(usize, &i64)> = input.iter().enumerate().collect();
        let len = input.len();
        for i in 0..len {
            let pos = i % len;
            let ind = enumerated.iter().position(|(i, _)| *i == pos).unwrap();
            let v = enumerated.remove(ind);
            let new_pos = (ind as i64 + 2 * (len - 1) as i64 + v.1) % (len - 1) as i64;
            if new_pos == 0 {
                enumerated.push(v);
            } else {
                enumerated.insert(new_pos as usize, v);
            }
            // println!(
            //     "{:?}",
            //     enumerated.iter().map(|v| v.1).collect::<Vec<&i64>>()
            // )
        }
        let zero_pos = enumerated.iter().position(|(_, v)| **v == 0).unwrap();

        let res = enumerated[(zero_pos + 1000) % len].1
            + enumerated[(zero_pos + 2000) % len].1
            + enumerated[(zero_pos + 3000) % len].1;
        Ok(res.to_string())
    }

    fn part2(input: Vec<i64>) -> Result<String> {
        let mut enumerated: Vec<(usize, i64)> =
            input.iter().map(|v| v * 811589153).enumerate().collect();
        let len = input.len();
        for _ in 0..10 {
            for i in 0..len {
                let pos = i % len;
                let ind = enumerated.iter().position(|(i, _)| *i == pos).unwrap();
                let v = enumerated.remove(ind);
                let new_pos =
                    (ind as i64 + 2 * 811589153 * (len - 1) as i64 + v.1) % (len - 1) as i64;
                if new_pos == 0 {
                    enumerated.push(v);
                } else {
                    enumerated.insert(new_pos as usize, v);
                }
            }
        }
        let zero_pos = enumerated.iter().position(|(_, v)| *v == 0).unwrap();

        let res = enumerated[(zero_pos + 1000) % len].1
            + enumerated[(zero_pos + 2000) % len].1
            + enumerated[(zero_pos + 3000) % len].1;
        Ok(res.to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn part1_test() -> Result<()> {
        let parsed_input = Day20::parse(INPUT)?;
        let result = Day20::part1(parsed_input)?;
        assert_eq!(&result, "3");
        Ok(())
    }

    #[test]
    fn part2_test() -> Result<()> {
        let parsed_input = Day20::parse(INPUT)?;
        let result = Day20::part2(parsed_input)?;
        assert_eq!(&result, "1623178306");
        Ok(())
    }

    const INPUT: &str = "1
2
-3
3
-2
0
4";
}
