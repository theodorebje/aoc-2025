use crate::{
    INPUT,
    shared::{Cell, Grid, MAX_ADJACENT_ROLLS, parse_input},
};
use aoc_2025_globals::{Output, Solution};

impl Grid {
    fn count_accessible_cells(&self) -> u32 {
        let mut accessible = 0;

        for (y, row) in self.0.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if matches!(cell, Cell::Roll) {
                    let count = Cell::rolls(self.adjacent(x, y));
                    if count < MAX_ADJACENT_ROLLS {
                        accessible += 1;
                    }
                }
            }
        }

        accessible
    }
}

pub struct Part1;

impl Solution for Part1 {
    type Answer = u32;

    fn run() -> Output<Self::Answer> {
        let grid: Grid = parse_input(INPUT);

        Output::new("Accessible rolls", grid.count_accessible_cells())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const MEDIUM_INPUT: &str = "....@
@@..@
@@@@@
..@@@
..@@.";
    const LARGE_INPUT: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    #[test]
    fn medium_count() {
        let grid: Grid = parse_input(MEDIUM_INPUT);
        assert_eq!(5, grid.count_accessible_cells());
    }

    #[test]
    fn large_count() {
        let grid: Grid = parse_input(LARGE_INPUT);
        assert_eq!(13, grid.count_accessible_cells());
    }
}
