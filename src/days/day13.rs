use crate::utils::{Day, Result, SplitString};
use serde_json::Value;
use std::cmp::Ordering;
use std::vec;

pub struct Day13 {}

impl Day13 {
    const DAY: u8 = 13;

    pub fn cmp(v1: &Value, v2: &Value) -> Ordering {
        match (v1, v2) {
            (Value::Number(n1), Value::Number(n2)) => {
                n1.as_u64().unwrap().cmp(&n2.as_u64().unwrap())
            }
            (Value::Array(a1), Value::Array(a2)) => {
                for i in 0..a1.len() {
                    let i1 = a1.get(i);
                    let i2 = a2.get(i);
                    let res = match (i1, i2) {
                        (Some(_), None) => Ordering::Greater,
                        (None, Some(_)) => Ordering::Less,
                        (None, None) => Ordering::Equal,
                        (Some(v1), Some(v2)) => Day13::cmp(v1, v2),
                    };
                    if res != Ordering::Equal {
                        return res;
                    }
                }
                if a1.len() == a2.len() {
                    Ordering::Equal
                } else {
                    Ordering::Less
                }
            }
            (Value::Array(_), Value::Number(_)) => Day13::cmp(v1, &Value::Array(vec![v2.clone()])),
            (Value::Number(_), Value::Array(_)) => Day13::cmp(&Value::Array(vec![v1.clone()]), v2),
            _ => panic!("Invalid value"),
        }
    }
}

impl Day<Vec<(Value, Value)>> for Day13 {
    fn day() -> u8 {
        Self::DAY
    }

    fn parse(input: &str) -> Result<Vec<(Value, Value)>> {
        Ok(input
            .split("\n\n")
            .map(|pair| {
                let (v1, v2) = pair.split_by("\n").unwrap();
                (
                    serde_json::from_str(v1).unwrap(),
                    serde_json::from_str(v2).unwrap(),
                )
            })
            .collect())
    }

    fn part1(input: Vec<(Value, Value)>) -> Result<String> {
        Ok(input
            .iter()
            .enumerate()
            .map(|(i, (v1, v2))| match Day13::cmp(v1, v2) {
                Ordering::Less => i + 1,
                _ => 0,
            })
            // .inspect(|v| println!("{}", v))
            .sum::<usize>()
            .to_string())
    }

    fn part2(input: Vec<(Value, Value)>) -> Result<String> {
        let mut vec: Vec<Value> = input
            .iter()
            .flat_map(|v| [v.0.clone(), v.1.clone()])
            .collect();

        let val1: Value = serde_json::from_str(r"[[2]]").unwrap();
        let val2: Value = serde_json::from_str(r"[[6]]").unwrap();
        vec.push(val1.clone());
        vec.push(val2.clone());
        vec.sort_by(Day13::cmp);
        let pos1 = vec.iter().position(|v| *v == val1).unwrap() + 1;
        let pos2 = vec.iter().position(|v| *v == val2).unwrap() + 1;

        Ok((pos1 * pos2).to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn part1_test() -> Result<()> {
        let parsed_input = Day13::parse(INPUT)?;
        let result = Day13::part1(parsed_input)?;
        assert_eq!(&result, "13");
        Ok(())
    }

    #[test]
    fn part2_test() -> Result<()> {
        let parsed_input = Day13::parse(INPUT)?;
        let result = Day13::part2(parsed_input)?;
        assert_eq!(&result, "140");
        Ok(())
    }

    const INPUT: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";
}
