use crate::{
    INPUT,
    shared::{Cell, Grid, MAX_ADJACENT_ROLLS, parse_input},
};
use aoc_2025_globals::{Output, Solution};
use std::num::NonZero;

impl Grid {
    fn remove_accessible_rolls(&mut self) -> Option<NonZero<u32>> {
        let clone = self.clone();
        let mut removed = 0;

        for (y, row) in self.0.iter_mut().enumerate() {
            for (x, cell) in row.iter_mut().enumerate() {
                if matches!(cell, Cell::Roll) {
                    let count = Cell::rolls(clone.adjacent(x, y));
                    if count < MAX_ADJACENT_ROLLS {
                        *cell = Cell::Empty;
                        removed += 1;
                    }
                }
            }
        }

        match removed {
            0 => None,
            1.. => Some(unsafe { NonZero::new(removed).unwrap_unchecked() }),
        }
    }
}

pub struct Part2;

impl Solution for Part2 {
    type Answer = u32;

    fn run() -> Output<Self::Answer> {
        let mut grid: Grid = parse_input(INPUT);

        let mut total_removed = 0;

        while let Some(removed) = grid.remove_accessible_rolls() {
            println!("Removed {removed} rolls");
            total_removed += removed.get();
        }

        Output::new("Removable rolls", total_removed)
    }
}
