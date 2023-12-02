#[derive(Debug, PartialEq)]
pub struct Day1 {
    solutions: Vec<(Option<char>, Option<char>, String)>,
}

impl Day1 {
    pub fn solve<S: AsRef<str>>(alt: bool, lines: &[S]) -> Self {
        let mut solutions = Vec::with_capacity(lines.len());
        for line in lines {
            solutions.push(Self::process_line(alt, line.as_ref()));
        }
        Self { solutions }
    }

    pub fn calibration(&self) -> i32 {
        let mut total = 0;
        for sol in self.solutions.iter() {
            let digits = (
                (sol.0.unwrap().to_string()).parse::<i32>().unwrap(),
                (sol.1.unwrap().to_string()).parse::<i32>().unwrap(),
            );
            total += digits.0 * 10 + digits.1;
        }
        total
    }

    fn process_line(alt: bool, line: &str) -> (Option<char>, Option<char>, String) {
        let mut index = 0;
        let mut first = None;
        let mut last = None;
        for c in line.chars() {
            let substr = &line[index..];
            if let Some(number) = Self::number_prefix(alt, substr) {
                if first.is_none() {
                    first = Some(number);
                }
                last = Some(number);
            }
            index += c.len_utf8();
        }
        (first, last, line.to_string())
    }

    fn number_prefix(alt: bool, s: &str) -> Option<char> {
        let digit = s.chars().nth(0).unwrap();
        if digit.is_ascii_digit() {
            return Some(digit);
        }
        if !alt {
            return None;
        }
        for (alt, digit) in [
            ("one", '1'),
            ("two", '2'),
            ("three", '3'),
            ("four", '4'),
            ("five", '5'),
            ("six", '6'),
            ("seven", '7'),
            ("eight", '8'),
            ("nine", '9'),
        ]
        .iter()
        {
            if s.starts_with(alt) {
                return Some(*digit);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day1_examples() {
        assert_eq!(
            Day1::solve(false, &["1abc2"]),
            Day1 {
                solutions: vec![(Some('1'), Some('2'), "1abc2".to_string())]
            }
        );
        assert_eq!(
            Day1::solve(false, &["pqr3stu8vwx"]),
            Day1 {
                solutions: vec![(Some('3'), Some('8'), "pqr3stu8vwx".to_string())]
            }
        );
        assert_eq!(
            Day1::solve(false, &["a1b2c3d4e5f"]),
            Day1 {
                solutions: vec![(Some('1'), Some('5'), "a1b2c3d4e5f".to_string())]
            }
        );
        assert_eq!(
            Day1::solve(false, &["treb7uchet"]),
            Day1 {
                solutions: vec![(Some('7'), Some('7'), "treb7uchet".to_string())]
            }
        );
    }

    #[test]
    fn day1_examples_calibrtation() {
        assert_eq!(Day1::solve(false, &["1abc2"]).calibration(), 12);
        assert_eq!(Day1::solve(false, &["pqr3stu8vwx"]).calibration(), 38);
        assert_eq!(Day1::solve(false, &["a1b2c3d4e5f"]).calibration(), 15);
        assert_eq!(Day1::solve(false, &["treb7uchet"]).calibration(), 77);
    }

    #[test]
    fn day1_examples_calibrtation_total() {
        assert_eq!(
            Day1::solve(
                false,
                &["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"]
            )
            .calibration(),
            12 + 38 + 15 + 77
        );
    }
}
