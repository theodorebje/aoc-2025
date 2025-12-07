use crate::INPUT;
use aoc_2025_globals::{Output, Solution};
use std::mem::MaybeUninit;

const NUM_BATTERIES: usize = 12;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Bank([u8; NUM_BATTERIES]);

impl From<Bank> for u64 {
    fn from(val: Bank) -> Self {
        val.0
            .iter()
            .fold(0, |acc, &digit| acc * 10 + Self::from(digit))
    }
}

impl Bank {
    fn sum(banks: impl Iterator<Item = Self>) -> u64 {
        banks.map(u64::from).sum()
    }
}

#[derive(Debug)]
struct EmptyBank;

struct BankScaffolding {
    maximum: u8,
    maximum_index: usize,
}

impl Bank {
    fn find_largest(pool: &[u8], respect: usize) -> Result<BankScaffolding, EmptyBank> {
        let mut index = None;
        let mut maximum = 0;
        for (bindex, battery) in pool[..pool.len() - respect].iter().enumerate() {
            if *battery > maximum {
                maximum = *battery;
                index = Some(bindex);
                if maximum == 9 {
                    break;
                }
            }
        }
        Ok(BankScaffolding {
            maximum,
            maximum_index: index.ok_or(EmptyBank)?,
        })
    }
}

impl TryFrom<&[u8]> for Bank {
    type Error = EmptyBank;

    fn try_from(pool: &[u8]) -> Result<Self, Self::Error> {
        let mut remaining = pool;
        let mut array: [MaybeUninit<u8>; NUM_BATTERIES] = [MaybeUninit::uninit(); NUM_BATTERIES];
        for (respect, element) in array.iter_mut().enumerate() {
            let BankScaffolding {
                maximum,
                maximum_index,
            } = Self::find_largest(remaining, NUM_BATTERIES - respect - 1)?;
            remaining = &remaining[maximum_index + 1..];
            *element = MaybeUninit::new(maximum);
        }

        // SAFETY: Since we used a for loop over the array, all the elements have been initialised.
        Ok(Self(unsafe { MaybeUninit::array_assume_init(array) }))
    }
}

fn str_to_bankable(input: &str) -> Vec<u8> {
    input
        .as_bytes()
        .iter()
        .map(|num| num - b'0')
        .collect::<Vec<u8>>()
}

pub struct Part2;

impl Solution for Part2 {
    type Answer = u64;

    fn run() -> Output<Self::Answer> {
        let banks = INPUT
            .trim_ascii_end()
            .split('\n')
            .map(|line| Bank::try_from(str_to_bankable(line).as_slice()).unwrap());

        let sum = Bank::sum(banks);

        Output::new("Maximum joltage", sum)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full() {
        assert_eq!(
            Bank([9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1]),
            Bank::try_from(str_to_bankable("987654321111111").as_slice()).unwrap()
        );
        assert_eq!(
            Bank([8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9]),
            Bank::try_from(str_to_bankable("811111111111119").as_slice()).unwrap()
        );
        assert_eq!(
            Bank([4, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8]),
            Bank::try_from(str_to_bankable("234234234234278").as_slice()).unwrap()
        );
        assert_eq!(
            Bank([8, 8, 8, 9, 1, 1, 1, 1, 2, 1, 1, 1]),
            Bank::try_from(str_to_bankable("818181911112111").as_slice()).unwrap()
        );
    }

    #[test]
    fn from() {
        assert_eq!(
            987_654_321_111,
            u64::from(Bank([9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1]))
        );
        assert_eq!(
            811_111_111_119,
            u64::from(Bank([8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9]))
        );
        assert_eq!(
            434_234_234_278,
            u64::from(Bank([4, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8]),)
        );
        assert_eq!(
            888_911_112_111,
            u64::from(Bank([8, 8, 8, 9, 1, 1, 1, 1, 2, 1, 1, 1]))
        );
    }

    #[test]
    fn add() {
        let banks = vec![
            Bank([9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1]),
            Bank([8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9]),
            Bank([4, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8]),
            Bank([8, 8, 8, 9, 1, 1, 1, 1, 2, 1, 1, 1]),
        ];
        assert_eq!(3_121_910_778_619, Bank::sum(banks.into_iter()));
    }
}
