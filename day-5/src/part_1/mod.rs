use crate::{INPUT, shared::parse_input};
use aoc_2025_globals::{Output, Solution};

pub struct Part1;

impl Solution for Part1 {
    type Answer = usize;

    fn run() -> Output<Self::Answer> {
        let (fresh, ingredients) = parse_input(INPUT);

        let mut counter = 0;

        for ingredient in ingredients.0 {
            if fresh.0.iter().any(|range| range.contains(&ingredient)) {
                counter += 1;
            }
        }

        Output::new("Fresh ingredients available", counter)
    }
}
