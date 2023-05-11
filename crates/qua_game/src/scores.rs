use serde::{Deserialize, Serialize};
use std::{ops::{Add, AddAssign, Sub, SubAssign}, fmt::Display};

#[derive(PartialEq, Eq, Default, Serialize, Deserialize, Clone, Copy, Debug)]
pub struct Scores {
    value: i32,
}

impl Display for Scores {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<i32> for Scores {
    fn from(value: i32) -> Self {
        Self { value }
    }
}

impl Add for Scores {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut out = self;
        out.value += rhs.value;
        out
    }
}

impl AddAssign for Scores {
    fn add_assign(&mut self, rhs: Self) {
        self.value += rhs.value;
    }
}

impl Sub for Scores {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut out = self;
        out.value -= rhs.value;
        out
    }
}

impl SubAssign for Scores {
    fn sub_assign(&mut self, rhs: Self) {
        self.value -= rhs.value;
    }
}
