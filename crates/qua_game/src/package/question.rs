use serde::{Deserialize, Serialize};

use crate::{person::PersonName, scores::Scores};

#[derive(Clone, Serialize, Deserialize)]
pub struct Question {
    pub answered: bool,
    pub answered_by: Option<PersonName>,
    pub cost: Scores,
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct QuestionIndex {
    index: usize,
}

impl From<usize> for QuestionIndex {
    fn from(item: usize) -> Self {
        Self { index: item }
    }
}

impl QuestionIndex {
    pub fn idx(&self) -> usize {
        self.index
    }
}
