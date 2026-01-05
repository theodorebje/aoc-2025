use std::{collections::HashSet, ops::RangeInclusive};

pub type Id = u64;

pub struct FreshSet(pub HashSet<RangeInclusive<Id>>);
pub struct IngredientSet(pub HashSet<Id>);

pub fn parse_input(input: &'static str) -> (FreshSet, IngredientSet) {
    let (fresh_str, ingredients_str) = input.split_once("\n\n").unwrap();

    let fresh_set = FreshSet(
        fresh_str
            .lines()
            .map(|line| {
                let (fx, tx) = line.split_once('-').unwrap();
                let f = fx.parse().unwrap();
                let t = tx.parse().unwrap();
                f..=t
            })
            .collect::<HashSet<_>>(),
    );

    let ingredient_set = IngredientSet(
        ingredients_str
            .lines()
            .map(|line| line.parse().unwrap())
            .collect::<HashSet<_>>(),
    );

    (fresh_set, ingredient_set)
}
