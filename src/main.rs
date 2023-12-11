mod day1;
mod day2;
mod day3;

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
            1 => println!(
                "calibration: {}",
                day1::Day1::solve(self.alt, &self.input()).calibration()
            ),
            2 => println!("sum: {}", day2::Day2::solve(&self.input()).sum(self.alt)),
            3 => println!("min: {:?}", day3::Day3::solve(&self.input()).sum(self.alt)),
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
