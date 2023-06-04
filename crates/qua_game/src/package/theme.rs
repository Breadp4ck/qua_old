use serde::{Deserialize, Serialize};

use super::question::QuestionState;

pub type ThemeIndex = usize;

#[derive(Clone, Serialize, Deserialize)]
pub struct ThemeState {
    pub questions: Vec<QuestionState>,
}
