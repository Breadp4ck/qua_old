use serde::{Deserialize, Serialize};

use crate::game::Question;

use super::round::Round;

#[derive(Clone, Serialize, Deserialize)]
pub struct Package {
    pub rounds: Vec<Round>,
    pub final_round: Round,
}

impl Package {
    pub fn mark_answered(&mut self, question: &Question) {
        match question {
            Question::Final(_) => {}
            Question::Normal(round_idx, theme_idx, question_idx) => {
                self.rounds[round_idx.idx()]
                    .themes[theme_idx.idx()]
                    .questions[question_idx.idx()].answered = true;
            }
        }
    }
}

impl Default for Package {
    fn default() -> Self {
        Self {
            rounds: vec![],
            final_round: Round { themes: vec![] },
        }
    }
}
