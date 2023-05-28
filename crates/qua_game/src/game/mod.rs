use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use std::collections::HashMap;

use crate::package::prelude::*;
use crate::person::prelude::*;

pub mod client_server;
pub mod game_context;
pub mod game_events;
pub mod game_inputs;
pub mod states;

pub use game_context::*;
pub use game_events::*;
pub use game_inputs::*;
pub use states::*;

#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum Round {
    Normal(RoundIndex),
    Final,
}

#[derive(PartialEq, Eq, Serialize, Deserialize, Clone, Copy)]
pub enum Theme {
    Normal(RoundIndex, ThemeIndex),
}

#[derive(Hash, PartialEq, Eq, Serialize, Deserialize, Clone, Copy)]
pub enum Question {
    Final(QuestionIndex),
    Normal(RoundIndex, ThemeIndex, QuestionIndex),
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

    pub fn package(&self) -> &PackageState {
        &self.package
    }

    pub fn sync(&mut self, new_game: Self) {
        self.package = new_game.package;
        self.state = new_game.state;
        self.persons = new_game.persons;
        self.context = new_game.context;

        self.context
            .events
            .push(GameEvent::Player(PlayerUpdate::Sync));
        self.context.events.push(GameEvent::Host(HostUpdate::Sync));
        self.context
            .events
            .push(GameEvent::Board(BoardUpdate::Text("КЕК".into())))

        // TODO: different board states
        // match self.state {
        //     GameState::Init(_) => todo!(),
        //     GameState::Greet(_) => todo!(),
        //     GameState::Overview(_) => todo!(),
        //     GameState::Board(_) => todo!(),
        //     GameState::QuestionAppearance(_) => todo!(),
        //     GameState::QuestionMatter(_) => todo!(),
        //     GameState::QuestionAsking(_) => todo!(),
        //     GameState::QuestionQuaWaiting(_) => todo!(),
        //     GameState::QuestionQuaQueue(_) => todo!(),
        //     GameState::QuestionQuaAnswering(_) => todo!(),
        //     GameState::QuestionAnswer(_) => todo!(),
        // }
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

    pub fn get_host(&self) -> Option<Host> {
        if let Some(name) = &self.context.host {
            if let Person::Host(host) = self.persons[name].clone() {
                Some(host)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn abandon_events(&mut self) -> bool {
        let event_count = self.context.events.len();
        self.context.events.clear();

        event_count > 0
    }

    pub fn event_try_recv(&mut self) -> Option<GameEvent> {
        self.context.events.pop()
    }

    pub fn add_person(&mut self, person: Person) {
        if person.is_host() {
            self.context.host = Some(person.name().clone());
            self.persons.insert(person.name().clone(), person.clone());
            self.context
                .events
                .push(GameEvent::Host(HostUpdate::Connected {
                    name: person.name().clone(),
                }));
        } else {
            self.persons.insert(person.name().clone(), person.clone());
            self.context
                .events
                .push(GameEvent::Player(PlayerUpdate::Connected {
                    name: person.name().clone(),
                }));
        }
    }

    pub fn remove_person(&mut self, name: PersonName) {
        let person = self.persons.remove(&name).unwrap();
        if person.is_host() {
            self.context
                .events
                .push(GameEvent::Host(HostUpdate::Disconnected));
        } else {
            self.context
                .events
                .push(GameEvent::Player(PlayerUpdate::Disconnected {
                    name: person.name().clone(),
                }));
        }
    }

    pub fn handle_input(&mut self, event: &InputEvent, author: &PersonName) {
        if let Some(state) = self.state.handle_input(
            &mut self.context,
            event,
            author,
            &mut self.persons,
            &mut self.package,
        ) {
            self.state = state;
            self.state.proceed_translation_event(&mut self.context);
        }
    }

    pub fn handle_stateless_input(&mut self, event: &StatelessInputEvent, author: &PersonName) {
        let person = self.persons.get(author).unwrap();
        match (event, person) {
            (StatelessInputEvent::AssignLeadPlayer(name), Person::Host(_)) => {
                self.context.lead = Some(name.clone());
                self.context
                    .events
                    .push(GameEvent::Player(PlayerUpdate::BecomeLead {
                        name: name.clone(),
                    }));
            }
            (StatelessInputEvent::GivePlayerScores(name, scores), Person::Host(_)) => {
                if let Person::Player(player) = self.persons.get_mut(name).unwrap() {
                    player.add_scores(scores.clone());
                    self.context
                        .events
                        .push(GameEvent::Player(PlayerUpdate::ScoresChanges {
                            name: player.name().clone(),
                            change: ScoreChange::Add { amount: *scores },
                            new_scores: player.scores(),
                        }));
                }
            }
            (StatelessInputEvent::TakePlayerScores(name, scores), Person::Host(_)) => {
                if let Person::Player(player) = self.persons.get_mut(name).unwrap() {
                    player.remove_scores(scores.clone());
                    self.context
                        .events
                        .push(GameEvent::Player(PlayerUpdate::ScoresChanges {
                            name: player.name().clone(),
                            change: ScoreChange::Remove { amount: *scores },
                            new_scores: player.scores(),
                        }));
                }
            }
            (StatelessInputEvent::SetPlayerScores(name, scores), Person::Host(_)) => {
                if let Person::Player(player) = self.persons.get_mut(name).unwrap() {
                    player.set_scores(scores.clone());
                    self.context
                        .events
                        .push(GameEvent::Player(PlayerUpdate::ScoresChanges {
                            name: player.name().clone(),
                            change: ScoreChange::Set { from: *scores },
                            new_scores: player.scores(),
                        }));
                }
            }
            _ => (),
        }
    }
}

pub mod prelude {
    pub use super::*;

    pub use client_server::*;
    pub use game_context::*;
    pub use game_events::*;
    pub use game_inputs::*;
    pub use states::*;
}
