use crate::utils::{Day, Result};
use std::collections::HashSet;

pub struct Day9 {}

impl Day9 {
    const DAY: u8 = 9;
}

#[derive(Clone)]
pub enum Action {
    Up(i64),
    Down(i64),
    Left(i64),
    Right(i64),
}

impl Action {
    pub fn value(&self) -> i64 {
        match self {
            Action::Up(v) => *v,
            Action::Down(v) => *v,
            Action::Left(v) => *v,
            Action::Right(v) => *v,
        }
    }
}

#[derive(Clone, Debug)]
struct Pos {
    x: i64,
    y: i64,
}

impl Pos {
    pub fn left(&mut self) {
        self.x -= 1;
    }

    pub fn right(&mut self) {
        self.x += 1;
    }

    pub fn up(&mut self) {
        self.y += 1;
    }

    pub fn down(&mut self) {
        self.y -= 1;
    }
}

struct Knot {
    head: Pos,
    tail: Vec<Pos>,
    visited: HashSet<String>,
}

impl Knot {
    pub fn new(tail_size: usize) -> Self {
        const INIT_POS: Pos = Pos { x: 0, y: 0 };
        let mut knot = Self {
            head: INIT_POS,
            tail: vec![INIT_POS; tail_size],
            visited: HashSet::new(),
        };
        knot.visited.insert(knot.to_string());
        knot
    }

    pub fn move_head(&mut self, action: &Action) {
        let count = action.value();
        for _ in 0..count {
            match action {
                Action::Up(_) => self.head.up(),
                Action::Down(_) => self.head.down(),
                Action::Left(_) => self.head.left(),
                Action::Right(_) => self.head.right(),
            }
            self.move_tail();
        }
    }

    pub fn move_tail(&mut self) {
        let mut prev = self.head.clone();
        for i in 0..self.tail.len() {
            let mut tail = self.tail[i].clone();
            if (tail.x - prev.x).abs() > 1 || (tail.y - prev.y).abs() > 1 {
                tail.x += prev.x.cmp(&tail.x) as i64;
                tail.y += prev.y.cmp(&tail.y) as i64;
            }
            self.tail[i] = tail.clone();
            prev = tail;
        }

        self.visited.insert(self.to_string());
    }
}

impl ToString for Knot {
    fn to_string(&self) -> String {
        let last = self.tail.last().unwrap();
        format!("{}_{}", last.x, last.y)
    }
}

impl Action {
    pub fn from_string(s: &str) -> Self {
        let spl: Vec<&str> = s.split_whitespace().collect();
        let dir = spl[0];
        let count = spl[1].parse::<i64>().unwrap();
        match dir {
            "U" => Action::Up(count),
            "D" => Action::Down(count),
            "L" => Action::Left(count),
            "R" => Action::Right(count),
            _ => panic!(),
        }
    }
}

impl Day<Vec<Action>> for Day9 {
    fn day() -> u8 {
        Self::DAY
    }

    fn parse(input: &str) -> Result<Vec<Action>> {
        Ok(input.lines().map(Action::from_string).collect())
    }

    fn part1(input: Vec<Action>) -> Result<String> {
        let mut knot = Knot::new(1);
        input.iter().for_each(|action| knot.move_head(action));
        Ok(knot.visited.len().to_string())
    }

    fn part2(input: Vec<Action>) -> Result<String> {
        let mut knot = Knot::new(9);
        input.iter().for_each(|action| knot.move_head(action));
        Ok(knot.visited.len().to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn part1_test() -> Result<()> {
        let parsed_input = Day9::parse(INPUT_1)?;
        let result = Day9::part1(parsed_input)?;
        assert_eq!(&result, "13");
        Ok(())
    }

    #[test]
    fn part2_test() -> Result<()> {
        let parsed_input = Day9::parse(INPUT_2)?;
        let result = Day9::part2(parsed_input)?;
        assert_eq!(&result, "36");
        Ok(())
    }

    const INPUT_1: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    const INPUT_2: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";
}
