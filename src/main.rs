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
            1 => println!("{:?}", Day1::solve(&self.input())),
            day => unimplemented!("the solution for day {} is missing", day),
        }
    }

    fn input(&self) -> Vec<String> {
        let mut lines = Vec::new();
        if let Some(ref single) = self.single {
            lines.push(single.clone())
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
    solutions: Vec<(Option<char>, Option<char>)>,
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
                    } else {
                        last = Some(c)
                    }
                }
            }
            solutions.push((first, last));
        }
        Self { solutions }
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
}
