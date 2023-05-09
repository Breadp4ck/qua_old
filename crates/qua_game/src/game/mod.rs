use serde::{Deserialize, Serialize};
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

#[derive(Serialize, Deserialize)]
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
pub enum GameEventLocal {
    Begin,
    SelectQuestion(Question),
    Answer,
    Score,
    Skip,
    Timeout,
}

#[derive(Serialize, Deserialize)]
pub enum GameEventGlobal {
    AssignLeadPlayer(PersonName),
    GivePlayerScores(PersonName, Scores),
    TakePlayerScores(PersonName, Scores),
    SetPlayerScores(PersonName, Scores),
    KickPlayer(PersonName),
}

#[derive(Serialize, Deserialize)]
pub enum GameEvent {
    Local(GameEventLocal),
    Global(GameEventGlobal),
}

trait GameStateInteraction {
    fn handle_event(
        &mut self,
        context: &mut GameContext,
        event: &GameEventLocal,
        author: &mut Person,
    ) -> Option<GameState>;
}

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
    fn handle_event(
        &mut self,
        context: &mut GameContext,
        event: &GameEventLocal,
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

pub struct GameContext {
    pub round: Round,
    pub lead_player: Option<PersonName>,
    pub question: Option<Question>,
}

impl Default for GameContext {
    fn default() -> Self {
        Self {
            round: Round::Default(0.into()),
            lead_player: None,
            question: None,
        }
    }
}

pub struct Game {
    package: Package,
    state: GameState,
    host: Option<Host>,
    players: HashMap<PersonName, Player>,
    context: GameContext,
}

impl Game {
    pub fn new(package: Package) -> Self {
        Game {
            package,
            state: GameState::Init(InitGameState::default()),
            host: None,
            players: HashMap::new(),
            context: GameContext::default(),
        }
    }

    pub fn add_host(&mut self, host: Host) {
        self.host = Some(host);
    }

    pub fn add_player(&mut self, player: Player) {
        self.players.insert(player.name().clone(), player);
    }

    pub fn remove_player(&mut self, name: PersonName) {
        self.players.remove(&name);
    }

    pub fn handle_event(&mut self, event: &GameEvent, author: &mut Person) {
        match event {
            GameEvent::Local(event) => {
                self.state.handle_event(&mut self.context, &event, author);
            }
            GameEvent::Global(event) => {
                self.handle_global_event(&event, author);
            }
        }
    }

    fn handle_global_event(&mut self, _event: &GameEventGlobal, _author: &mut Person) {}
}

pub mod prelude {
    pub use super::*;
}
