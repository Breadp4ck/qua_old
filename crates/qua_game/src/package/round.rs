use serde::{Deserialize, Serialize};

use super::theme::ThemeState;

#[derive(Clone, Serialize, Deserialize)]
pub struct RoundState {
    pub themes: Vec<ThemeState>,
}

#[derive(Hash, PartialEq, Eq, Serialize, Deserialize, Clone, Copy)]
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
