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

impl ToString for PersonName {
    fn to_string(&self) -> String {
        self.data.clone()
    }
}

impl Into<String> for PersonName {
    fn into(self) -> String {
        self.data
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
