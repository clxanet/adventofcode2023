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
        println!("{self:?}");
    }
}
