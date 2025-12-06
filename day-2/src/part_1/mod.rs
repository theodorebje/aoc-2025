use crate::shared::parse::parse_input;
use aoc_2025_globals::{Output, Solution};

pub struct Part1;

impl Solution for Part1 {
    type Answer = u64;

    fn run() -> Output<Self::Answer> {
        let ranges = parse_input();
        let mut all = vec![];
        for range in ranges {
            for i in range {
                let s = i.to_string();
                let len = s.len();
                if len % 2 == 0 {
                    let (a, b) = s.split_at(len / 2);
                    if a == b {
                        all.push(i);
                    }
                }
            }
        }
        let sum: u64 = all.iter().sum();
        Output::new("Sum of invalid product IDs", sum)
    }
}
