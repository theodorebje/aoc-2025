use crate::shared::commands::{Command, Direction};
use std::{
    ops::{Add, Sub},
    sync::Mutex,
};

static ZERO_COUNT: Mutex<u32> = Mutex::new(0);

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
}

impl Default for Dial {
    fn default() -> Self {
        Self::new()
    }
}

fn add_mutex() {
    let mut data = ZERO_COUNT.lock().expect("unable to grab lock");
    *data += 1;
}

pub fn get_count() -> u32 {
    *ZERO_COUNT.lock().expect("unable to grab lock")
}

impl Add<u32> for Dial {
    type Output = Self;

    fn add(self, rhs: u32) -> Self {
        let mut num: u32 = u32::from(self.0);
        for _ in 0..rhs {
            num += 1;
            if num == 100 {
                num = 0;
                add_mutex();
            }
        }
        #[allow(clippy::cast_possible_truncation)]
        Self(num as u8)
    }
}

impl Sub<u32> for Dial {
    type Output = Self;

    fn sub(self, rhs: u32) -> Self {
        let mut num: u32 = u32::from(self.0);
        for _ in 0..rhs {
            num = num.checked_sub(1).unwrap_or(99);
            if num == 0 {
                add_mutex();
            }
        }
        #[allow(clippy::cast_possible_truncation)]
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
