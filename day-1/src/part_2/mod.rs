use self::dial::{Dial, get_count};
use crate::{INPUT, shared::commands};
use aoc_2025_globals::{Output, Solution};
use std::str::FromStr;

mod dial;

fn parse_input() -> Vec<commands::Command> {
    INPUT
        .split_ascii_whitespace()
        .filter_map(|x| commands::Command::from_str(x).ok())
        .collect()
}

pub struct Part2;
impl Solution for Part2 {
    type Answer = u32;

    fn run() -> aoc_2025_globals::Output<Self::Answer> {
        let input = parse_input();
        let mut dial = Dial::new();
        for command in input {
            dial = dial.apply(command);
        }
        let count = get_count();
        Output::new("Password", count)
    }
}
