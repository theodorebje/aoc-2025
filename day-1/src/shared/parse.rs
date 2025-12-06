use crate::{INPUT, shared::commands};
use std::str::FromStr;

#[must_use]
pub fn parse_input() -> Vec<commands::Command> {
    INPUT
        .split_ascii_whitespace()
        .filter_map(|x| commands::Command::from_str(x).ok())
        .collect()
}
