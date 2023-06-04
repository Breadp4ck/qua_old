use serde::{Deserialize, Serialize};

use crate::game::Question;

use super::{round::RoundState, question::QuestionState};

#[derive(Clone, Serialize, Deserialize)]
pub struct PackageState {
    pub rounds: Vec<RoundState>,
    pub final_round: RoundState,
}

impl PackageState {
    pub fn mark_answered(&mut self, question: &Question) {
        match *question {
            Question::Final(_) => {}
            Question::Normal(round_idx, theme_idx, question_idx) => {
                self.rounds[round_idx]
                    .themes[theme_idx]
                    .questions[question_idx].answered = true;
            }
        }
    }

    pub fn get(&self, question: &Question) -> &QuestionState{
        match *question {
            Question::Final(_) => todo!(),
            Question::Normal(round_idx, theme_idx, question_idx) => {
                &self.rounds[round_idx]
                    .themes[theme_idx]
                    .questions[question_idx]
            }
        }

    }
}

impl Default for PackageState {
    fn default() -> Self {
        Self {
            rounds: vec![],
            final_round: RoundState { themes: vec![] },
        }
    }
}
