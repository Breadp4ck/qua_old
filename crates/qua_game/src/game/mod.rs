use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use std::collections::HashMap;

use crate::package::prelude::*;
use crate::person::prelude::*;
use crate::person::*;
use crate::scores::Scores;

mod board_game_state;
mod greet_game_state;
mod init_game_state;
mod overview_game_state;
mod question_answer_game_state;
mod question_appearance_game_state;
mod question_asking_game_state;
mod question_matter_game_state;
mod question_qua_answering_game_state;
mod question_qua_queue_game_state;
mod question_qua_waiting_game_state;

use board_game_state::*;
use greet_game_state::*;
use init_game_state::*;
use overview_game_state::*;
use question_answer_game_state::*;
use question_appearance_game_state::*;
use question_asking_game_state::*;
use question_matter_game_state::*;
use question_qua_answering_game_state::*;
use question_qua_queue_game_state::*;
use question_qua_waiting_game_state::*;

#[derive(PartialEq)]
pub enum EmiterType {
    Host,
    Lead,
    Player,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum Round {
    Default(RoundIndex),
    Final,
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum Question {
    Final(QuestionIndex),
    Normal(RoundIndex, ThemeIndex, QuestionIndex),
}

#[derive(Serialize, Deserialize)]
pub enum InputEvent {
    Begin,
    SelectQuestion(Question),
    Answer,
    Score,
    Skip,
    Timeout,
}

#[derive(Serialize, Deserialize)]
pub enum StatelessInputEvent {
    AssignLeadPlayer(PersonName),
    GivePlayerScores(PersonName, Scores),
    TakePlayerScores(PersonName, Scores),
    SetPlayerScores(PersonName, Scores),
    KickPlayer(PersonName),
}

#[derive(Serialize, Deserialize)]
pub enum StateEvent {}

#[derive(Serialize, Deserialize)]
pub enum Command {
    StatelessInput(StatelessInputEvent),
    Input(InputEvent),
    SyncRequest,
    SyncResponse(Game),
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

trait GameStateInteraction {
    fn handle_event(
        &mut self,
        context: &mut GameContext,
        event: &InputEvent,
        author: &mut Person,
    ) -> Option<GameState>;
}

#[derive(Clone, Serialize, Deserialize)]
pub enum GameState {
    Init(InitGameState),
    Greet(GreetGameState),
    Overview(OverviewGameState),
    Board(BoardGameState),
    QuestionAppearance(QuestionAppearanceGameState),
    QuestionMatter(QuestionMatterGameState),
    QuestionAsking(QuestionAskingGameState),
    QuestionQuaWaiting(QuestionQuaWaitingGameState),
    QuestionQuaQueue(QuestionQuaQueueGameState),
    QuestionQuaAnswering(QuestionQuaAnsweringGameState),
    QuestionAnswer(QuestionAnswerGameState),
}

impl GameState {
    fn handle_input(
        &mut self,
        context: &mut GameContext,
        event: &InputEvent,
        author: &mut Person,
    ) -> Option<Self> {
        match self {
            GameState::Init(state) => state.handle_event(context, event, author),
            GameState::Greet(state) => state.handle_event(context, event, author),
            GameState::Overview(state) => state.handle_event(context, event, author),
            GameState::Board(state) => state.handle_event(context, event, author),
            GameState::QuestionAppearance(state) => state.handle_event(context, event, author),
            GameState::QuestionMatter(state) => state.handle_event(context, event, author),
            GameState::QuestionAsking(state) => state.handle_event(context, event, author),
            GameState::QuestionQuaWaiting(state) => state.handle_event(context, event, author),
            GameState::QuestionQuaQueue(state) => state.handle_event(context, event, author),
            GameState::QuestionQuaAnswering(state) => state.handle_event(context, event, author),
            GameState::QuestionAnswer(state) => state.handle_event(context, event, author),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GameContext {
    pub round: Round,
    pub host: Option<PersonName>,
    pub lead: Option<PersonName>,
    pub question: Option<Question>,
}

impl Default for GameContext {
    fn default() -> Self {
        Self {
            round: Round::Default(0.into()),
            host: None,
            lead: None,
            question: None,
        }
    }
}

#[serde_as]
#[derive(Clone, Serialize, Deserialize)]
pub struct Game {
    package: Package,
    state: GameState,
    #[serde_as(as = "Vec<(_, _)>")]
    persons: HashMap<PersonName, Person>,
    context: GameContext,
}

impl Game {
    pub fn new(package: Package) -> Self {
        Game {
            package,
            state: GameState::Init(InitGameState::default()),
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
            &event,
            self.persons.get_mut(author).unwrap(),
        );
    }

    pub fn handle_stateless_input(&mut self, _event: &StatelessInputEvent, _author: &PersonName) {}
}

pub mod prelude {
    pub use super::*;
}
