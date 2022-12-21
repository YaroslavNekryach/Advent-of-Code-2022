use crate::utils::{parse_int, Day, Result, SplitString};
use regex::Regex;
use std::collections::HashMap;
use std::ops::Index;

pub struct Day21 {}

impl Day21 {
    const DAY: u8 = 21;

    fn get_value(mut map: HashMap<String, Monkey>, name: String) -> (u64, HashMap<String, Monkey>) {
        let mut monkey = map.remove(&name).unwrap();
        match monkey.value {
            Some(v) => {
                let name = monkey.name.clone();
                map.insert(name, monkey);
                (v, map)
            }
            None => {
                let operation = monkey.operation.clone().unwrap();
                let (v1, new_map) = Self::get_value(map, operation.operand1);
                map = new_map;
                let (v2, new_map) = Self::get_value(map, operation.operand2);
                map = new_map;
                let result = match operation.action {
                    Action::Mul => v1 * v2,
                    Action::Add => v1 + v2,
                    Action::Div => v1 / v2,
                    Action::Sub => v1 - v2,
                };
                monkey.value = Some(result);
                let name = monkey.name.clone();
                map.insert(name, monkey);
                (result, map)
            }
        }
    }
}
#[derive(Clone, Debug)]
pub enum Action {
    Add,
    Sub,
    Mul,
    Div,
}

impl Action {
    pub fn from_string(s: &str) -> Self {
        if s.contains('+') {
            Action::Add
        } else if s.contains('*') {
            Action::Mul
        } else if s.contains('/') {
            Action::Div
        } else if s.contains('-') {
            Action::Sub
        } else {
            panic!("Invalid action")
        }
    }
}

#[derive(Clone)]
pub struct Operation {
    operand1: String,
    operand2: String,
    action: Action,
}

impl Operation {
    // pub fn exec(&self) -> u64 {
    //     match self.action {
    //         Action::Add => self.operand1.value(old) + self.operand2.value(old),
    //         Action::Mul => self.operand1.value(old) * self.operand2.value(old),
    //     }
    // }

    pub fn from_string(s: &str) -> Option<Self> {
        let re = Regex::new(r"^(.*) [*+-\\] (.*)$").unwrap();
        let captures = re.captures(s)?;
        Some(Self {
            operand1: captures.index(1).to_string(),
            operand2: captures.index(2).to_string(),
            action: Action::from_string(s),
        })
    }
}

#[derive(Clone)]
pub struct Monkey {
    name: String,
    value: Option<u64>,
    operation: Option<Operation>,
}
impl Day<HashMap<String, Monkey>> for Day21 {
    fn day() -> u8 {
        Self::DAY
    }

    fn parse(input: &str) -> Result<HashMap<String, Monkey>> {
        Ok(input.lines().fold(HashMap::new(), |mut buffer, line| {
            let (name, action) = line.split_by(": ").unwrap();
            let operation = Operation::from_string(action);
            let monkey = Monkey {
                name: name.to_string(),
                value: if operation.is_some() {
                    None
                } else {
                    Some(action.parse().unwrap())
                },
                operation,
            };
            buffer.insert(name.to_string(), monkey);
            buffer
        }))
    }

    fn part1(input: HashMap<String, Monkey>) -> Result<String> {
        return Ok("0".to_string());
        Ok(Day21::get_value(input, "root".to_string()).0.to_string())
    }

    fn part2(mut input: HashMap<String, Monkey>) -> Result<String> {
        let humn = input.get_mut("humn").unwrap();
        humn.value = Some(3887609741189);
        let (result, map) = Day21::get_value(input, "root".to_string());
        let root = map.get("root").unwrap();
        let op = root.operation.clone().unwrap();
        let v1 = map.get(&op.operand1).unwrap();
        let v2 = map.get(&op.operand2).unwrap();
        println!("{} {}", v1.value.unwrap(), v2.value.unwrap());
        println!("{}", v1.value.unwrap() - v2.value.unwrap());
        Ok(result.to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn part1_test() -> Result<()> {
        let parsed_input = Day21::parse(INPUT)?;
        let result = Day21::part1(parsed_input)?;
        assert_eq!(&result, "152");
        Ok(())
    }

    #[test]
    fn part2_test() -> Result<()> {
        let parsed_input = Day21::parse(INPUT)?;
        let result = Day21::part2(parsed_input)?;
        assert_eq!(&result, "301");
        Ok(())
    }

    const INPUT: &str = "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32";
}
