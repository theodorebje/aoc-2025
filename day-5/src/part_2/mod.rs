use crate::{
    INPUT,
    shared::{FreshSet, Id, parse_input},
};
use aoc_2025_globals::{Output, Solution};
use std::cmp::max;

fn fresh_calc(fresh: &FreshSet) -> Id {
    let mut ranges: Vec<(Id, Id)> = fresh
        .0
        .iter()
        .map(|range| (range.start, range.last))
        .collect();
    ranges.sort_unstable();
    let ranges = ranges;

    let mut counter = 0;

    let mut previous_last = 0;

    for (start, end) in ranges {
        let from = max(start, previous_last);
        if from > end {
            continue;
        }
        counter += end - from + 1;
        previous_last = end + 1;
    }

    counter
}

pub struct Part2;

impl Solution for Part2 {
    type Answer = Id;

    fn run() -> Output<Self::Answer> {
        let (fresh, _) = parse_input(INPUT);

        Output::new(
            "Max number of fresh ingredients possible",
            fresh_calc(&fresh),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = "3-5
10-14
16-20
12-18\n\n";
    const EXAMPLE_ANSWER: Id = 14;
    const EXAMPLE_2: &str = "1-3
7-9
5-8
24-42\n\n";
    const EXAMPLE_2_ANSWER: Id = 27;

    #[test]
    fn a() {
        let (fresh, _) = parse_input(EXAMPLE);
        let result = dbg!(fresh_calc(&fresh));
        assert_eq!(EXAMPLE_ANSWER, result);
    }

    #[test]
    fn b() {
        let (fresh, _) = parse_input(EXAMPLE_2);
        let result = dbg!(fresh_calc(&fresh));
        assert_eq!(EXAMPLE_2_ANSWER, result);
    }

    #[test]
    fn answer() {
        const ANS: Id = 365_804_144_481_581;

        assert_eq!(ANS, fresh_calc(&parse_input(INPUT).0));
    }
}
