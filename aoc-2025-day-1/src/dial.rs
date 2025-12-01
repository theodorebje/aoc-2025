use std::ops::{Add, Sub};

use crate::commands::{Command, Direction};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Dial(u8);

impl Dial {
    pub const fn new() -> Self {
        Self(50)
    }

    pub fn apply(self, command: Command) -> Self {
        match command.direction {
            Direction::Right => self.add(command.distance),
            Direction::Left => self.sub(command.distance),
        }
    }

    pub const fn is_zero(self) -> bool {
        self.0 == 0
    }
}

impl Default for Dial {
    fn default() -> Self {
        Self::new()
    }
}

impl Add<u32> for Dial {
    type Output = Self;

    fn add(self, rhs: u32) -> Self {
        let simple = (rhs + u32::from(self.0)) % 100;
        Self(simple as u8)
    }
}

impl Sub<u32> for Dial {
    type Output = Self;

    fn sub(self, rhs: u32) -> Self {
        // Rewrite this in idiomatic Rust.
        let mut num: u32 = u32::from(self.0);
        for _ in 0..rhs {
            num = num.checked_sub(1).unwrap_or(99);
        }
        Self(num as u8)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_simple() {
        assert_eq!(Dial::new() + 10, Dial(60));
        assert_eq!(Dial::new() + 30, Dial(80));
        assert_eq!(Dial::new() + 49, Dial(99));
    }
    #[test]
    fn add_wrap() {
        assert_eq!(Dial::new() + 50, Dial(0));
        assert_eq!(Dial::new() + 70, Dial(20));
        assert_eq!(Dial::new() + 100, Dial(50));
        assert_eq!(Dial::new() + 149, Dial(99));
    }
    #[test]
    fn add_double_wrap() {
        assert_eq!(Dial::new() + 150, Dial(0));
        assert_eq!(Dial::new() + 158, Dial(8));
    }
    #[test]
    fn add_ridiculous_wrap() {
        assert_eq!(Dial::new() + 1000, Dial(50));
        assert_eq!(Dial::new() + 10000, Dial(50));
    }

    #[test]
    fn sub_simple() {
        assert_eq!(Dial::new() - 10, Dial(40));
        assert_eq!(Dial::new() - 30, Dial(20));
        assert_eq!(Dial::new() - 50, Dial(0));
    }
    #[test]
    fn sub_wrap() {
        assert_eq!(Dial::new() - 70, Dial(80));
        assert_eq!(Dial::new() - 100, Dial(50));
    }
}
