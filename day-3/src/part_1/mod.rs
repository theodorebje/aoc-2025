use crate::INPUT;
use aoc_2025_globals::{Output, Solution};

#[derive(Debug, Clone, Copy)]
struct Bank {
    first: u8,
    second: u8,
}

impl From<Bank> for u32 {
    fn from(val: Bank) -> Self {
        (val.first * 10 + val.second).into()
    }
}

#[derive(Debug)]
struct EmptyBank;

impl TryFrom<&[u8]> for Bank {
    type Error = EmptyBank;

    fn try_from(pool: &[u8]) -> Result<Self, Self::Error> {
        let mut index = None;
        let mut biggest = 0;
        for (bindex, battery) in pool[..pool.len() - 1].iter().enumerate() {
            if *battery > biggest {
                biggest = *battery;
                index = Some(bindex);
                if biggest == 9 {
                    break;
                }
            }
        }
        let second = pool[index.ok_or(EmptyBank)? + 1..]
            .iter()
            .max()
            .ok_or(EmptyBank)?;
        Ok(Self {
            first: biggest,
            second: *second,
        })
    }
}

pub struct Part1;

impl Solution for Part1 {
    type Answer = u32;

    fn run() -> Output<Self::Answer> {
        let banks = INPUT.trim_ascii_end().split('\n').map(|line| {
            let batteries: Vec<u8> = line.bytes().map(|num| num - b'0').collect();
            Bank::try_from(batteries.as_slice()).unwrap()
        });

        let sum: u32 = banks.map(u32::from).sum();

        Output::new("Maximum joltage", sum)
    }
}
