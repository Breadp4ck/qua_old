use std::ops::{Add, AddAssign, Sub, SubAssign};
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, Default, Serialize, Deserialize, Clone, Copy, Debug)]
pub struct Scores {
    value: i32,
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
