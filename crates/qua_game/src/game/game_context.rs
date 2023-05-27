use serde::{Deserialize, Serialize};

use super::{Question, Round, GameEvent};

use crate::person::prelude::*;

#[derive(Clone, Serialize, Deserialize)]
pub struct GameContext {
    pub events: Vec<GameEvent>,
    pub round: Round,
    pub host: Option<PersonName>,
    pub lead: Option<PersonName>,
    pub question: Option<Question>,
}

impl Default for GameContext {
    fn default() -> Self {
        Self {
            events: vec![],
            round: Round::Normal(0.into()),
            host: None,
            lead: None,
            question: None,
        }
    }
}
