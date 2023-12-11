pub struct Day3 {
    #[allow(unused)]
    numbers: Vec<Number>,

    #[allow(unused)]
    symbols: Vec<Symbol>,
}

#[derive(Debug, PartialEq)]
struct Number {
    value: u32,
    x: (usize, usize),
    y: usize,
}

#[derive(Debug, PartialEq)]
struct Symbol {
    value: char,
    x: usize,
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

        let symbols = lines
            .iter()
            .enumerate()
            .flat_map(|(y, line)| symbols_in_line(y, line))
            .collect();
        Self { numbers, symbols }
    }

    fn part_numbers(&self) -> Vec<u32> {
        let mut ret = vec![];
        for number in self.numbers.iter() {
            if self.symbols.iter().any(|s| number.is_adjacent(s)) {
                ret.push(number.value);
            }
        }
        ret
    }

    pub fn sum(&self) -> u32 {
        self.part_numbers().into_iter().sum()
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

    fn is_adjacent(&self, symbol: &Symbol) -> bool {
        let mut xs = (self.x.0)..=(self.x.1);
        coord_is_adjacent(self.y, symbol.y) && xs.any(|x| coord_is_adjacent(x, symbol.x))
    }
}

fn coord_is_adjacent(a: usize, b: usize) -> bool {
    if let Some(x) = a.checked_sub(b) {
        x <= 1
    } else if let Some(x) = b.checked_sub(a) {
        x <= 1
    } else {
        false
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
    if let Some(n) = number {
        numbers.push(n)
    }
    numbers.into_iter()
}

fn symbols_in_line<S: AsRef<str>>(y: usize, line: S) -> impl Iterator<Item = Symbol> {
    let mut symbols = vec![];
    for (x, c) in line.as_ref().chars().enumerate() {
        if is_symbol(c) {
            symbols.push(Symbol { value: c, x, y })
        }
    }
    symbols.into_iter()
}

fn is_symbol(c: char) -> bool {
    !(c.is_ascii_digit() || c == '.')
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
    fn symbol_between_numbers() {
        let solution = Day3::solve(&["841*168"]);
        assert_eq!(
            solution.numbers,
            vec![
                Number {
                    value: 841,
                    x: (0, 2),
                    y: 0
                },
                Number {
                    value: 168,
                    x: (4, 6),
                    y: 0
                }
            ]
        );
    }

    #[test]
    fn example() {
        let solution = Day3::solve(EXAMPLE);
        assert_eq!(
            solution.part_numbers(),
            vec![467, 35, 633, 617, 592, 755, 664, 598]
        );
        assert_eq!(solution.sum(), 4361)
    }

    #[test]
    fn single_digit_adjacent() {
        let number = Number {
            value: 0,
            x: (1, 1),
            y: 1,
        };
        for x in 0..=2 {
            for y in 0..=2 {
                let symbol = Symbol { value: '+', x, y };
                assert!(dbg!(&number).is_adjacent(dbg!(&symbol)));
            }
        }
    }

    #[test]
    fn triple_digit_adjacent() {
        let number = Number {
            value: 123,
            x: (1, 3),
            y: 1,
        };
        for x in 0..=4 {
            for y in 0..=2 {
                let symbol = Symbol { value: '+', x, y };
                assert!(dbg!(&number).is_adjacent(dbg!(&symbol)));
            }
        }
    }

    #[test]
    fn is_a_symbol() {
        assert!(is_symbol('#'));
        assert!(is_symbol('+'));
        assert!(is_symbol('-'));
        assert!(is_symbol('!'));

        assert!(!is_symbol('1'));
        assert!(!is_symbol('1'));
        assert!(!is_symbol('0'));
    }

    #[test]
    fn parse_symbols() {
        let solution = Day3::solve(EXAMPLE);
        assert_eq!(
            solution.symbols,
            vec![
                Symbol {
                    value: '*',
                    x: 3,
                    y: 1,
                },
                Symbol {
                    value: '#',
                    x: 6,
                    y: 3,
                },
                Symbol {
                    value: '*',
                    x: 3,
                    y: 4,
                },
                Symbol {
                    value: '+',
                    x: 5,
                    y: 5,
                },
                Symbol {
                    value: '$',
                    x: 3,
                    y: 8,
                },
                Symbol {
                    value: '*',
                    x: 5,
                    y: 8,
                },
            ]
        )
    }

    #[test]
    fn parse_numbers() {
        let solution = Day3::solve(EXAMPLE);
        assert_eq!(
            solution.numbers,
            vec![
                Number {
                    value: 467,
                    x: (0, 2),
                    y: 0,
                },
                Number {
                    value: 114,
                    x: (5, 7),
                    y: 0,
                },
                Number {
                    value: 35,
                    x: (2, 3),
                    y: 2,
                },
                Number {
                    value: 633,
                    x: (6, 8),
                    y: 2,
                },
                Number {
                    value: 617,
                    x: (0, 2),
                    y: 4,
                },
                Number {
                    value: 58,
                    x: (7, 8),
                    y: 5,
                },
                Number {
                    value: 592,
                    x: (2, 4),
                    y: 6,
                },
                Number {
                    value: 755,
                    x: (6, 8),
                    y: 7,
                },
                Number {
                    value: 664,
                    x: (1, 3),
                    y: 9,
                },
                Number {
                    value: 598,
                    x: (5, 7),
                    y: 9,
                }
            ]
        )
    }
}
