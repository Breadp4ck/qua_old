use crate::{scores::Scores, person::PersonName};

pub struct Question {
    pub answered: bool,
    pub answered_by: Vec<PersonName>,
    pub cost: Scores,
}

#[derive(Clone, Copy)]
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
