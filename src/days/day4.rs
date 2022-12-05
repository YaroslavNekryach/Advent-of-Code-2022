use crate::utils::{get_input, print_result, SplitString};

const DAY: u8 = 4;

pub fn exec() {
    let input = get_input(DAY).unwrap();
    let parsed_input = parse(&input);
    print_result(1, &part1(&parsed_input));
    print_result(2, &part2(&parsed_input));
}

#[derive(Debug)]
struct Section {
    from: u64,
    to: u64,
}

#[derive(Debug)]
struct Pair {
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
}

impl TryFrom<&str> for Section {
    type Error = std::num::ParseIntError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let (p1, p2) = s.split_by("-").unwrap();
        Ok(Self {
            from: p1.parse()?,
            to: p2.parse()?,
        })
    }
}

impl TryFrom<&str> for Pair {
    type Error = std::num::ParseIntError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let (p1, p2) = s.split_by(",").unwrap();
        Ok(Self {
            section1: p1.try_into()?,
            section2: p2.try_into()?,
        })
    }
}

fn parse(input: &str) -> Vec<Pair> {
    input.lines().map(|v| v.try_into().unwrap()).collect()
}

fn part1(input: &[Pair]) -> String {
    part(input, Pair::contains)
}

fn part2(input: &[Pair]) -> String {
    part(input, Pair::overlap)
}

fn part(input: &[Pair], check: fn(&Pair) -> bool) -> String {
    input
        .iter()
        .map(check)
        .map(|c| c as u64)
        .sum::<u64>()
        .to_string()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn part1_test() {
        let parsed_input = parse(INPUT);
        let result = part1(&parsed_input);
        assert_eq!(&result, "2")
    }

    #[test]
    fn part2_test() {
        let parsed_input = parse(INPUT);
        let result = part2(&parsed_input);
        assert_eq!(&result, "4")
    }

    const INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
}
