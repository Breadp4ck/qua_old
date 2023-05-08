use std::fmt::Display;

use rand::{distributions::Alphanumeric, Rng};
use serde::{Serialize, Deserialize};

use crate::MAX_TICKET_ID_LENGTH;


#[derive(Hash, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct Ticket {
    id: String,
}

impl Ticket {
    pub fn random() -> Self {
        let rng = rand::thread_rng();
        let id = rng
            .sample_iter(Alphanumeric)
            .take(MAX_TICKET_ID_LENGTH)
            .map(char::from)
            .collect();

        Self { id }
    }
}

impl From<String> for Ticket {
    fn from(id: String) -> Self {
        Self { id }
    }
}

impl Display for Ticket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}
