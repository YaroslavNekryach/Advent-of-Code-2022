use crate::utils::{Day, Result, SplitString};

pub struct Day4 {}

impl Day4 {
    const DAY: u8 = 4;

    fn part(input: &[Pair], check: fn(&Pair) -> bool) -> String {
        input
            .iter()
            .map(check)
            .map(|c| c as u64)
            .sum::<u64>()
            .to_string()
    }
}

impl Day<Vec<Pair>> for Day4 {
    fn day() -> u8 {
        Self::DAY
    }

    fn parse(input: &str) -> Result<Vec<Pair>> {
        Ok(input
            .lines()
            .map(|v| Pair::from_string(v).unwrap())
            .collect())
    }

    fn part1(input: Vec<Pair>) -> Result<String> {
        Ok(Self::part(&input, Pair::contains))
    }

    fn part2(input: Vec<Pair>) -> Result<String> {
        Ok(Self::part(&input, Pair::overlap))
    }
}

#[derive(Clone, Debug)]
pub struct Section {
    from: u64,
    to: u64,
}

impl Section {
    fn from_string(s: &str) -> Result<Self> {
        let (p1, p2) = s.split_by("-").unwrap();
        Ok(Self {
            from: p1.parse()?,
            to: p2.parse()?,
        })
    }
}

#[derive(Clone, Debug)]
pub struct Pair {
    section1: Section,
    section2: Section,
}

impl Pair {
    fn contains(&self) -> bool {
        self.section1.from >= self.section2.from && self.section1.to <= self.section2.to
            || self.section2.from >= self.section1.from && self.section2.to <= self.section1.to
    }

    fn overlap(&self) -> bool {
        !(self.section1.to < self.section2.from && self.section1.from < self.section2.from
            || self.section2.to < self.section1.from && self.section2.from < self.section1.from)
    }

    fn from_string(s: &str) -> Result<Self> {
        let (p1, p2) = s.split_by(",").unwrap();
        Ok(Self {
            section1: Section::from_string(p1)?,
            section2: Section::from_string(p2)?,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn part1_test() {
        let parsed_input = Day4::parse(INPUT).unwrap();
        let result = Day4::part1(parsed_input).unwrap();
        assert_eq!(&result, "2")
    }

    #[test]
    fn part2_test() {
        let parsed_input = Day4::parse(INPUT).unwrap();
        let result = Day4::part2(parsed_input).unwrap();
        assert_eq!(&result, "4")
    }

    const INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
}
