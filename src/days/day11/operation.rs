use regex::Regex;
use std::ops::Index;

#[derive(Clone, Debug)]
pub struct Operation {
    operand1: Value,
    operand2: Value,
    action: Action,
}

impl Operation {
    pub fn exec(&self, old: u64) -> u64 {
        match self.action {
            Action::Add => self.operand1.value(old) + self.operand2.value(old),
            Action::Mul => self.operand1.value(old) * self.operand2.value(old),
        }
    }

    pub fn from_string(s: &str) -> Self {
        let re = Regex::new(r"new = (.*) [*+] (.*)").unwrap();
        let captures = re.captures(s).unwrap();
        Self {
            operand1: Value::from_string(captures.index(1)),
            operand2: Value::from_string(captures.index(2)),
            action: Action::from_string(s),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Value {
    Old,
    Number(u64),
}

impl Value {
    pub fn value(&self, old: u64) -> u64 {
        match self {
            Value::Old => old,
            Value::Number(value) => *value,
        }
    }

    pub fn from_string(s: &str) -> Self {
        if s == "old" {
            Value::Old
        } else {
            Value::Number(s.parse().unwrap())
        }
    }
}

#[derive(Clone, Debug)]
pub enum Action {
    Add,
    Mul,
}

impl Action {
    pub fn from_string(s: &str) -> Self {
        if s.contains('+') {
            Action::Add
        } else if s.contains('*') {
            Action::Mul
        } else {
            panic!("Invalid action")
        }
    }
}
