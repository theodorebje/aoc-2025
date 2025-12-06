use std::fmt::Display;

pub struct Output<T: Display> {
    pub label: &'static str,
    pub value: T,
}

impl<T: Display> Output<T> {
    pub fn print(&self) {
        println!("{}: {}", self.label, self.value);
    }

    pub const fn new(label: &'static str, value: T) -> Self {
        Self { label, value }
    }
}

pub trait Solution {
    type Answer: Display;
    fn run() -> Output<Self::Answer>;
}
