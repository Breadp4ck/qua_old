use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use std::collections::HashMap;

use crate::package::prelude::*;
use crate::person::prelude::*;

mod game_context;
mod states;

use states::prelude::*;
use game_context::GameContext;

#[derive(Clone, Serialize, Deserialize)]
pub enum Round {
    Normal(RoundIndex),
    Final,
}

#[derive(Hash, PartialEq, Eq, Serialize, Deserialize, Clone, Copy)]
pub enum Question {
    Final(QuestionIndex),
    Normal(RoundIndex, ThemeIndex, QuestionIndex),
}

#[derive(Serialize, Deserialize)]
pub enum ClientMessage {
    Input(InputEvent),
    StatelessInput(StatelessInputEvent),
    SyncRequest,
}

#[derive(Serialize, Deserialize)]
pub enum ServerMessage {
    Input(InputEvent, PersonName),
    StatelessInput(StatelessInputEvent, PersonName),
    SyncResponse(Game),
    PersonConnected(Person),
    PersonDisconnected(PersonName),
}

#[serde_as]
#[derive(Clone, Serialize, Deserialize)]
pub struct Game {
    package: PackageState,
    state: GameState,
    #[serde_as(as = "Vec<(_, _)>")]
    persons: HashMap<PersonName, Person>,
    context: GameContext,
}

impl Game {
    pub fn new(package: PackageState) -> Self {
        Game {
            package,
            state: GameState::default(),
            persons: HashMap::new(),
            context: GameContext::default(),
        }
    }

    pub fn sync(&mut self, new_game: Self) {
        self.package = new_game.package;
        self.state = new_game.state;
        self.persons = new_game.persons;
        self.context = new_game.context;
    }

    pub fn get_players(&self) -> Vec<Player> {
        let mut players: Vec<Player> = vec![];

        for person in self.persons.values() {
            if let Person::Player(player) = person {
                players.push(player.clone());
            }
        }

        players
    }

    pub fn add_person(&mut self, person: Person) {
        self.persons.insert(person.name().clone(), person);
    }

    pub fn remove_person(&mut self, name: PersonName) {
        self.persons.remove(&name);
    }

    pub fn handle_input(&mut self, event: &InputEvent, author: &PersonName) {
        self.state.handle_input(
            &mut self.context,
            event,
            author,
            &mut self.persons,
            &mut self.package
        );
    }

    pub fn handle_stateless_input(&mut self, _event: &StatelessInputEvent, _author: &PersonName) {}
}

pub mod prelude {
    pub use super::*;
}
