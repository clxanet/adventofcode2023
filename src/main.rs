use std::path::PathBuf;

use clap::Parser;

fn main() {
    Args::parse().execute()
}

#[derive(Debug, Parser)]
#[command(about, version, name = "adventofcode2023")]
struct Args {
    #[clap(short, long)]
    single: Option<String>,

    #[clap(short, long)]
    path: Option<PathBuf>,

    #[clap(short, long)]
    alt: bool,

    #[clap()]
    day: u8,
}

impl Args {
    fn execute(self) {
        match self.day {
            1 => println!("calibration: {}", Day1::solve(&self.input()).calibration()),
            day => unimplemented!("the solution for day {} is missing", day),
        }
    }

    fn input(&self) -> Vec<String> {
        let mut lines = Vec::new();
        if let Some(ref single) = self.single {
            lines.push(single.clone())
        }
        if let Some(ref path) = self.path {
            let content = std::fs::read_to_string(path).unwrap();
            lines.extend(
                content
                    .split('\n')
                    .map(|line| line.trim().to_string())
                    .filter(|s| !s.is_empty()),
            );
        }
        assert!(
            !lines.is_empty(),
            "please provide a single line input or an input file path"
        );
        lines
    }
}

#[derive(Debug, PartialEq)]
struct Day1 {
    solutions: Vec<(Option<char>, Option<char>, String)>,
}

impl Day1 {
    fn solve<S: AsRef<str>>(lines: &[S]) -> Self {
        let mut solutions = Vec::with_capacity(lines.len());
        for line in lines {
            let mut first = None;
            let mut last = None;
            for c in line.as_ref().chars() {
                if c.is_ascii_digit() {
                    if first.is_none() {
                        first = Some(c)
                    }
                    last = Some(c)
                }
            }
            solutions.push((first, last, line.as_ref().to_string()));
        }
        Self { solutions }
    }

    fn calibration(&self) -> i32 {
        let mut total = 0;
        for sol in self.solutions.iter() {
            println!("{sol:?}");
            let digits = (
                (sol.0.unwrap().to_string()).parse::<i32>().unwrap(),
                (sol.1.unwrap().to_string()).parse::<i32>().unwrap(),
            );
            total += digits.0 * 10 + digits.1;
        }
        total
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day1_examples() {
        assert_eq!(
            Day1::solve(&["1abc2"]),
            Day1 {
                solutions: vec![(Some('1'), Some('2'))]
            }
        );
        assert_eq!(
            Day1::solve(&["pqr3stu8vwx"]),
            Day1 {
                solutions: vec![(Some('3'), Some('8'))]
            }
        );
        assert_eq!(
            Day1::solve(&["a1b2c3d4e5f"]),
            Day1 {
                solutions: vec![(Some('1'), Some('5'))]
            }
        );
        assert_eq!(
            Day1::solve(&["treb7uchet"]),
            Day1 {
                solutions: vec![(Some('7'), Some('7'))]
            }
        );
    }

    #[test]
    fn day1_examples_calibrtation() {
        assert_eq!(Day1::solve(&["1abc2"]).calibration(), 12);
        assert_eq!(Day1::solve(&["pqr3stu8vwx"]).calibration(), 38);
        assert_eq!(Day1::solve(&["a1b2c3d4e5f"]).calibration(), 15);
        assert_eq!(Day1::solve(&["treb7uchet"]).calibration(), 77);
    }

    #[test]
    fn day1_examples_calibrtation_total() {
        assert_eq!(
            Day1::solve(&["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"]).calibration(),
            12 + 38 + 15 + 77
        );
    }
}
