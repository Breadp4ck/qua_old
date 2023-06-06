use serde::{Deserialize, Serialize};

use crate::{game::Question, prelude::Round};

use super::{question::QuestionState, round::RoundState};

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
                self.rounds[round_idx].themes[theme_idx].questions[question_idx].answered = true;
            }
        }
    }

    pub fn get(&self, question: &Question) -> &QuestionState {
        match *question {
            Question::Final(_) => todo!(),
            Question::Normal(round_idx, theme_idx, question_idx) => {
                &self.rounds[round_idx].themes[theme_idx].questions[question_idx]
            }
        }
    }

    pub fn is_last(&self, round: &Round) -> bool {
        match round {
            Round::Normal(idx) => {
                if *idx == self.rounds.len() - 1 {
                    return true
                }
            }
            Round::Final => todo!(),
        }

        false
    }

    pub fn is_clear(&self, round: &Round) -> bool {
        match round {
            Round::Normal(idx) => {
                for theme in &self.rounds[*idx].themes {
                    for question in &theme.questions {
                        if !question.answered {
                            return false;
                        }
                    }
                }
            }
            Round::Final => todo!(),
        }

        true
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
