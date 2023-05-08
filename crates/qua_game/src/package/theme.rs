use serde::{Serialize, Deserialize};

use super::question::Question;

pub struct Theme {
    pub questions: Vec<Question>,
}

#[derive(Serialize, Deserialize, Clone, Copy)]
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
