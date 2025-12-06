use clap::{Parser, Subcommand};
use pastey::paste;

/// Advent of Code 2025 code runner and test suite
#[derive(Debug, Parser)]
#[command(version, about)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
    #[arg(short, long)]
    pub day: u8,
    #[arg(short, long)]
    pub part: u8,
}

macro_rules! match_challenges {
    ($day:ident, $part:ident, [$($num:literal),* $(,)?]) => {
        paste! {
            match ($day, $part) {
                $(
                    ($num, 1) => [<aoc_2025_day_$num>]::part_1::Part1::run().print(),
                    ($num, 2) => [<aoc_2025_day_$num>]::part_2::Part2::run().print(),
                )*
                (day, part) => panic!("not a valid day or part number: day-{day}-part-{part}"),
            }
        }
    }
}

impl Args {
    pub fn entry(&self) {
        let day = self.day;
        let part = self.part;
        match_challenges!(day, part, []);
    }
}

#[derive(Debug, Subcommand)]
#[non_exhaustive]
pub enum Commands {
    Run,
}
