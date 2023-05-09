use super::{PersonName, Personality};
use crate::scores::Scores;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, Clone, Default, Serialize, Deserialize)]
pub struct Player {
    name: PersonName,
    scores: Scores,
}

impl Personality for Player {
    fn name(&self) -> &PersonName {
        &self.name
    }

    fn with_name(name: PersonName) -> Self {
        Self {
            name,
            scores: Scores::default(),
        }
    }
}

impl Player {
    pub fn scores(&self) -> Scores {
        self.scores
    }

    pub fn set_scores(&mut self, scores: Scores) {
        self.scores = scores
    }

    pub fn add_scores(&mut self, scores: Scores) {
        self.scores += scores
    }

    pub fn remove_scores(&mut self, scores: Scores) {
        self.scores -= scores
    }
}
