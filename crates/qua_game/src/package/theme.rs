use serde::{Deserialize, Serialize};

use super::question::QuestionState;

#[derive(Clone, Serialize, Deserialize)]
pub struct ThemeState {
    pub questions: Vec<QuestionState>,
}

#[derive(Hash, PartialEq, Eq, Serialize, Deserialize, Clone, Copy)]
pub struct ThemeIndex {
    index: usize,
}

impl From<usize> for ThemeIndex {
    fn from(item: usize) -> Self {
        Self { index: item }
    }
}

impl ThemeIndex {
    pub fn idx(&self) -> usize {
        self.index
    }
}
