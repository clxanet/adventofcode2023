pub struct Day3 {
    #[allow(unused)]
    numbers: Vec<Number>,
}

#[derive(Debug, PartialEq)]
struct Number {
    value: u32,
    x: (usize, usize),
    y: usize,
}

impl Day3 {
    pub fn solve<S: AsRef<str>>(lines: &[S]) -> Self {
        let _ = lines;
        let numbers = lines
            .iter()
            .enumerate()
            .flat_map(|(y, line)| numbers_in_line(y, line))
            .collect();
        Self { numbers }
    }

    pub fn min(&self) -> i32 {
        0
    }
}

impl Number {
    fn expand(&self, value: u32) -> Self {
        Self {
            value: self.value * 10 + value,
            y: self.y,
            x: (self.x.0, self.x.1 + 1),
        }
    }
}

fn numbers_in_line<S: AsRef<str>>(y: usize, line: S) -> impl Iterator<Item = Number> {
    let mut number = Option::<Number>::None;
    let mut numbers = vec![];
    for (i, c) in line.as_ref().chars().enumerate() {
        if c.is_ascii_digit() {
            if let Some(n) = number {
                number = Some(n.expand(c.to_digit(10).unwrap()))
            } else {
                number = Some(Number {
                    value: c.to_digit(10).unwrap(),
                    y,
                    x: (i, i),
                })
            }
        } else if let Some(n) = number {
            numbers.push(n);
            number = None;
        }
    }
    numbers.into_iter()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &[&str] = &[
        "467..114..",
        "...*......",
        "..35..633.",
        "......#...",
        "617*......",
        ".....+.58.",
        "..592.....",
        "......755.",
        "...$.*....",
        ".664.598..",
    ];

    #[test]
    fn parse_first_number() {
        let solution = Day3::solve(EXAMPLE);
        assert_eq!(
            solution.numbers[0],
            Number {
                value: 467,
                x: (0, 2),
                y: 0,
            }
        )
    }
}
