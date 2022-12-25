use crate::utils::{parse_int, Day, Result};

pub struct Day25 {}

impl Day25 {
    const DAY: u8 = 25;

    pub fn parse_number(s: &str) -> u64 {
        s.chars()
            .rev()
            .enumerate()
            .fold::<i64, _>(0, |res, (i, c)| {
                res + 5i64.pow(i as u32)
                    * match c {
                        '=' => -2,
                        '-' => -1,
                        '0' => 0,
                        '1' => 1,
                        '2' => 2,
                        _ => panic!(),
                    }
            }) as u64
    }

    pub fn to_string(mut v: u64) -> String {
        let mut result = String::new();
        v += 2;
        while v > 2 {
            let l: i64 = ((v as i64) % 5) - 2;
            v = v / 5 + 2;
            let c = match l {
                0 => '0',
                1 => '1',
                2 => '2',
                -2 => '=',
                -1 => '-',
                _ => panic!(),
            };
            result.push(c);
        }

        result.chars().rev().collect()
    }
}

impl Day<Vec<String>> for Day25 {
    fn day() -> u8 {
        Self::DAY
    }

    fn parse(input: &str) -> Result<Vec<String>> {
        Ok(input.lines().map(|v| v.to_string()).collect())
    }

    fn part1(input: Vec<String>) -> Result<String> {
        Ok(Day25::to_string(
            input.iter().map(|v| Day25::parse_number(v)).sum::<u64>(),
        ))
    }

    fn part2(input: Vec<String>) -> Result<String> {
        Ok("0".to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn part1_test() -> Result<()> {
        let parsed_input = Day25::parse(INPUT)?;
        let result = Day25::part1(parsed_input)?;
        assert_eq!(&result, "4890");
        Ok(())
    }

    #[test]
    fn parse_test() {
        assert_eq!(Day25::parse_number("1"), 1);
        assert_eq!(Day25::parse_number("2"), 2);
        assert_eq!(Day25::parse_number("1="), 3);
        assert_eq!(Day25::parse_number("1-"), 4);
        assert_eq!(Day25::parse_number("10"), 5);
        assert_eq!(Day25::parse_number("11"), 6);
        assert_eq!(Day25::parse_number("12"), 7);
        assert_eq!(Day25::parse_number("2="), 8);
        assert_eq!(Day25::parse_number("2-"), 9);
        assert_eq!(Day25::parse_number("20"), 10);
        assert_eq!(Day25::parse_number("1=0"), 15);
        assert_eq!(Day25::parse_number("1-0"), 20);
        assert_eq!(Day25::parse_number("1=11-2"), 2022);
        assert_eq!(Day25::parse_number("1-0---0"), 12345);
        assert_eq!(Day25::parse_number("1121-1110-1=0"), 314159265);
    }
    #[test]
    fn to_string_test() {
        assert_eq!(Day25::to_string(1), "1");
        assert_eq!(Day25::to_string(2), "2");
        assert_eq!(Day25::to_string(3), "1=");
        assert_eq!(Day25::to_string(4), "1-");
        assert_eq!(Day25::to_string(5), "10");
        assert_eq!(Day25::to_string(6), "11");
        assert_eq!(Day25::to_string(7), "12");
        assert_eq!(Day25::to_string(8), "2=");
        assert_eq!(Day25::to_string(9), "2-");
        assert_eq!(Day25::to_string(10), "20");
        assert_eq!(Day25::to_string(15), "1=0");
        assert_eq!(Day25::to_string(20), "1-0");
        assert_eq!(Day25::to_string(2022), "1=11-2");
        assert_eq!(Day25::to_string(12345), "1-0---0");
        assert_eq!(Day25::to_string(314159265), "1121-1110-1=0");
    }

    const INPUT: &str = "1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122";
}
