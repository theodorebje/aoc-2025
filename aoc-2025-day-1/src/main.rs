use std::str::FromStr;

mod commands;
mod dial;

const INPUT: &str = include_str!("../input.txt");

fn parse_input() -> Vec<commands::Command> {
    INPUT
        .split_ascii_whitespace()
        .filter_map(|x| commands::Command::from_str(x).ok())
        .collect()
}

fn main() {
    let input = parse_input();

    println!("{input:#?}");
}
