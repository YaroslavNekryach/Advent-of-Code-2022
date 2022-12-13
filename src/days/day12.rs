use crate::utils::{parse_int, Day, Result};

pub struct Day12 {}

impl Day12 {
    const DAY: u8 = 12;
}

#[derive(Clone, Copy, Debug)]
pub struct Pos {
    x: usize,
    y: usize,
}

impl Day<Vec<Vec<char>>> for Day12 {
    fn day() -> u8 {
        Self::DAY
    }

    fn parse(input: &str) -> Result<Vec<Vec<char>>> {
        Ok(input.lines().map(|line| line.chars().collect()).collect())
    }

    fn part1(input: Vec<Vec<char>>) -> Result<String> {
        let mut start: Pos = Pos { x: 0, y: 0 };
        let mut end: Pos = Pos { x: 0, y: 0 };
        let x_size = input[0].len();
        let y_size = input.len();

        for (yi, y) in input.iter().enumerate() {
            for (xi, x) in y.iter().enumerate() {
                if *x == 'S' {
                    start = Pos { x: xi, y: yi };
                }
                if *x == 'E' {
                    end = Pos { x: xi, y: yi };
                }
            }
        }

        let mut result_map = vec![vec![-1; x_size]; y_size];
        result_map[start.y][start.x] = 0;
        let mut queue: Vec<Pos> = vec![start];

        while !queue.is_empty() {
            let pos = queue.remove(0);
            if pos.x > 0 {
                let x = pos.x - 1;
                let y = pos.y;
                let mut from = input[pos.y][pos.x];
                let mut to = input[y][x];
                if from == 'S' {
                    from = 'a';
                }
                if from == 'E' {
                    from = 'z';
                }
                if to == 'S' {
                    to = 'a';
                }
                if to == 'E' {
                    to = 'z';
                }
                if (to as i32 - from as i32) <= 1 && result_map[y][x] < 0 {
                    result_map[y][x] = result_map[pos.y][pos.x] + 1;
                    queue.push(Pos { x, y });
                }
            }
            if pos.y > 0 {
                let x = pos.x;
                let y = pos.y - 1;
                let mut from = input[pos.y][pos.x];
                let mut to = input[y][x];
                if from == 'S' {
                    from = 'a';
                }
                if from == 'E' {
                    from = 'z';
                }
                if to == 'S' {
                    to = 'a';
                }
                if to == 'E' {
                    to = 'z';
                }
                if (to as i32 - from as i32) <= 1 && result_map[y][x] < 0 {
                    result_map[y][x] = result_map[pos.y][pos.x] + 1;
                    queue.push(Pos { x, y });
                }
            }
            if pos.x < x_size - 1 {
                let x = pos.x + 1;
                let y = pos.y;
                let mut from = input[pos.y][pos.x];
                let mut to = input[y][x];
                if from == 'S' {
                    from = 'a';
                }
                if from == 'E' {
                    from = 'z';
                }
                if to == 'S' {
                    to = 'a';
                }
                if to == 'E' {
                    to = 'z';
                }
                if (to as i32 - from as i32) <= 1 && result_map[y][x] < 0 {
                    result_map[y][x] = result_map[pos.y][pos.x] + 1;
                    queue.push(Pos { x, y });
                }
            }
            if pos.y < y_size - 1 {
                let x = pos.x;
                let y = pos.y + 1;
                let mut from = input[pos.y][pos.x];
                let mut to = input[y][x];
                if from == 'S' {
                    from = 'a';
                }
                if from == 'E' {
                    from = 'z';
                }
                if to == 'S' {
                    to = 'a';
                }
                if to == 'E' {
                    to = 'z';
                }
                if (to as i32 - from as i32) <= 1 && result_map[y][x] < 0 {
                    result_map[y][x] = result_map[pos.y][pos.x] + 1;
                    queue.push(Pos { x, y });
                }
            }
        }
        // result_map.iter().for_each(|v| println!("{:>3?}", v));
        Ok(result_map[end.y][end.x].to_string())
    }

    fn part2(input: Vec<Vec<char>>) -> Result<String> {
        let mut start: Pos = Pos { x: 0, y: 0 };
        let mut end: Pos = Pos { x: 0, y: 0 };
        let x_size = input[0].len();
        let y_size = input.len();

        for (yi, y) in input.iter().enumerate() {
            for (xi, x) in y.iter().enumerate() {
                if *x == 'S' {
                    start = Pos { x: xi, y: yi };
                }
                if *x == 'E' {
                    end = Pos { x: xi, y: yi };
                }
            }
        }

        let mut result_map = vec![vec![-1; x_size]; y_size];
        result_map[end.y][end.x] = 0;
        let mut queue: Vec<Pos> = vec![end];

        while !queue.is_empty() {
            let pos = queue.remove(0);
            if pos.x > 0 {
                let x = pos.x - 1;
                let y = pos.y;
                let mut from = input[pos.y][pos.x];
                let mut to = input[y][x];
                if from == 'S' {
                    from = 'a';
                }
                if from == 'E' {
                    from = 'z';
                }
                if to == 'S' {
                    to = 'a';
                }
                if to == 'E' {
                    to = 'z';
                }
                if (from as i32 - to as i32) <= 1 && result_map[y][x] < 0 {
                    result_map[y][x] = result_map[pos.y][pos.x] + 1;
                    queue.push(Pos { x, y });
                }
            }
            if pos.y > 0 {
                let x = pos.x;
                let y = pos.y - 1;
                let mut from = input[pos.y][pos.x];
                let mut to = input[y][x];
                if from == 'S' {
                    from = 'a';
                }
                if from == 'E' {
                    from = 'z';
                }
                if to == 'S' {
                    to = 'a';
                }
                if to == 'E' {
                    to = 'z';
                }
                if (from as i32 - to as i32) <= 1 && result_map[y][x] < 0 {
                    result_map[y][x] = result_map[pos.y][pos.x] + 1;
                    queue.push(Pos { x, y });
                }
            }
            if pos.x < x_size - 1 {
                let x = pos.x + 1;
                let y = pos.y;
                let mut from = input[pos.y][pos.x];
                let mut to = input[y][x];
                if from == 'S' {
                    from = 'a';
                }
                if from == 'E' {
                    from = 'z';
                }
                if to == 'S' {
                    to = 'a';
                }
                if to == 'E' {
                    to = 'z';
                }
                if (from as i32 - to as i32) <= 1 && result_map[y][x] < 0 {
                    result_map[y][x] = result_map[pos.y][pos.x] + 1;
                    queue.push(Pos { x, y });
                }
            }
            if pos.y < y_size - 1 {
                let x = pos.x;
                let y = pos.y + 1;
                let mut from = input[pos.y][pos.x];
                let mut to = input[y][x];
                if from == 'S' {
                    from = 'a';
                }
                if from == 'E' {
                    from = 'z';
                }
                if to == 'S' {
                    to = 'a';
                }
                if to == 'E' {
                    to = 'z';
                }
                if (from as i32 - to as i32) <= 1 && result_map[y][x] < 0 {
                    result_map[y][x] = result_map[pos.y][pos.x] + 1;
                    queue.push(Pos { x, y });
                }
            }
        }
        let mut res_vec: Vec<i32> = Vec::new();
        for (yi, y) in input.iter().enumerate() {
            for (xi, x) in y.iter().enumerate() {
                if *x == 'a' {
                    res_vec.push(result_map[yi][xi]);
                }
            }
        }
        result_map.iter().for_each(|v| println!("{:>3?}", v));

        Ok(res_vec
            .iter()
            .filter(|v| **v > 0)
            .min()
            .unwrap()
            .to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn part1_test() -> Result<()> {
        let parsed_input = Day12::parse(INPUT)?;
        let result = Day12::part1(parsed_input)?;
        assert_eq!(&result, "31");
        Ok(())
    }

    #[test]
    fn part2_test() -> Result<()> {
        let parsed_input = Day12::parse(INPUT)?;
        let result = Day12::part2(parsed_input)?;
        assert_eq!(&result, "29");
        Ok(())
    }

    const INPUT: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";
}
