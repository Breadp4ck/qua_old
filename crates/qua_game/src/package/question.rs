use serde::{Deserialize, Serialize};

use crate::{person::PersonName, scores::Scores};

pub type QuestionIndex = usize;

#[derive(Clone, Serialize, Deserialize)]
pub struct QuestionState {
    pub answered: bool,
    pub answered_by: Option<PersonName>,
    pub cost: Scores,
}
