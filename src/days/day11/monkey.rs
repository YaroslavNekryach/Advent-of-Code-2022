use crate::days::day11::operation::Operation;
use crate::utils::parse_int;

#[derive(Clone, Debug)]
pub struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    pub test: u64,
    success: usize,
    fail: usize,
    pub inspections: u64,
}

impl Monkey {
    pub fn test(&mut self, handler: Option<u64>) -> Option<(u64, usize)> {
        if self.items.is_empty() {
            return None;
        }

        let item = self.items.pop().unwrap();
        let mut new = self.operation.exec(item);
        new = match handler {
            None => new / 3,
            Some(v) => new % v,
        };
        self.inspections += 1;
        if new % self.test == 0 {
            Some((new, self.success))
        } else {
            Some((new, self.fail))
        }
    }

    pub fn add_item(&mut self, item: u64) {
        self.items.push(item)
    }

    pub fn from_string(s: &str) -> Self {
        let lines: Vec<&str> = s.lines().collect();
        let items = lines[1]
            .split_terminator(": ")
            .last()
            .unwrap()
            .split_terminator(", ")
            .map(parse_int)
            .collect();
        let operation = Operation::from_string(lines[2]);
        let test: u64 = lines[3]
            .split_terminator(" by ")
            .last()
            .unwrap()
            .parse()
            .unwrap();
        let success = lines[4]
            .split_terminator(" monkey ")
            .last()
            .unwrap()
            .parse()
            .unwrap();
        let fail = lines[5]
            .split_terminator(" monkey ")
            .last()
            .unwrap()
            .parse()
            .unwrap();

        Self {
            items,
            operation,
            test,
            success,
            fail,
            inspections: 0,
        }
    }
}
