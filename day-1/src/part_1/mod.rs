use self::dial::Dial;
use crate::shared::parse::parse_input;
use aoc_2025_globals::{Output, Solution};

mod dial;

pub struct Part1;

impl Solution for Part1 {
    type Answer = u32;

    fn run() -> Output<Self::Answer> {
        let input = parse_input();
        let mut dial = Dial::new();
        let mut count: u32 = 0;
        for command in input {
            dial = dial.apply(command);
            if dial.is_zero() {
                count += 1;
            }
        }
        Output::new("Password", count)
    }
}
