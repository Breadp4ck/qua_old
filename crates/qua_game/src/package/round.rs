use serde::{Deserialize, Serialize};

use super::theme::Theme;

#[derive(Clone, Serialize, Deserialize)]
pub struct Round {
    pub themes: Vec<Theme>,
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct RoundIndex {
    index: usize,
}

impl RoundIndex {
    pub fn new(index: usize) -> Self {
        Self { index }
    }

    pub fn first() -> Self {
        Self { index: 0 }
    }

    pub fn idx(&self) -> usize {
        self.index
    }
}

impl From<usize> for RoundIndex {
    fn from(item: usize) -> Self {
        Self { index: item }
    }
}
