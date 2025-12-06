use crate::INPUT;
use std::str::FromStr;

pub fn parse_input() -> impl Iterator<Item = std::ops::RangeInclusive<u64>> {
    INPUT.trim_ascii().split(',').map(|x| {
        let (a, b) = x.split_once('-').unwrap();
        let a = u64::from_str(a).unwrap();
        let b = u64::from_str(b).unwrap();
        a..=b
    })
}
