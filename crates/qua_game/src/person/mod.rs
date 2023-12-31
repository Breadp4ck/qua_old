use std::fmt::Display;

use serde::{Deserialize, Serialize};

mod host;
mod player;

pub trait Personality {
    fn name(&self) -> &PersonName;
    fn with_name(name: PersonName) -> Self;
}

#[derive(PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum Person {
    Player(player::Player),
    Host(host::Host),
}

impl Person {
    pub fn name(&self) -> &PersonName {
        match self {
            Person::Player(player) => player.name(),
            Person::Host(host) => host.name(),
        }
    }

    pub fn is_host(&self) -> bool {
        matches!(self, Person::Host(_))
    }

    pub fn is_player(&self) -> bool {
        matches!(self, Person::Player(_))
    }

}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash, Default)]
pub struct PersonName {
    data: String,
}

impl PersonName {
    pub fn new(name: &str) -> Self {
        Self {
            data: name.to_string(),
        }
    }
}

impl Into<String> for PersonName {
    fn into(self) -> String {
        self.data
    }
}

impl Display for PersonName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.data)
    }
}

pub mod prelude {
    pub use super::*;

    pub use super::Person;
    pub use super::PersonName;
    pub use super::Personality;

    pub use host::*;
    pub use player::*;
}
