use std::collections::HashMap;
use std::time::Duration;

use serde::{Deserialize, Serialize};

use crate::package::prelude::*;
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

use super::Question;
use super::game_context::GameContext;

type Persons = HashMap<PersonName, Person>;

#[derive(Serialize, Deserialize)]
pub enum InputEvent {
    Begin,
    SelectQuestion(Question),
    Answer(Duration),
    CountCorrect,
    CountWrong,
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

trait GameStateInteraction {
    fn handle_event(
        &mut self,
        context: &mut GameContext,
        event: &InputEvent,
        author: &PersonName,
        persons: &mut Persons,
        package: &mut PackageState,
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
    pub fn handle_input(
        &mut self,
        context: &mut GameContext,
        event: &InputEvent,
        author: &PersonName,
        persons: &mut Persons,
        package: &mut PackageState,
    ) -> Option<Self> {
        match self {
            GameState::Init(state) => state.handle_event(context, event, author, persons, package),
            GameState::Greet(state) => state.handle_event(context, event, author, persons, package),
            GameState::Overview(state) => state.handle_event(context, event, author, persons, package),
            GameState::Board(state) => state.handle_event(context, event, author, persons, package),
            GameState::QuestionAppearance(state) => state.handle_event(context, event, author, persons, package),
            GameState::QuestionMatter(state) => state.handle_event(context, event, author, persons, package),
            GameState::QuestionAsking(state) => state.handle_event(context, event, author, persons, package),
            GameState::QuestionQuaWaiting(state) => state.handle_event(context, event, author, persons, package),
            GameState::QuestionQuaQueue(state) => state.handle_event(context, event, author, persons, package),
            GameState::QuestionQuaAnswering(state) => state.handle_event(context, event, author, persons, package),
            GameState::QuestionAnswer(state) => state.handle_event(context, event, author, persons, package),
        }
    }
}

impl Default for GameState {
    fn default() -> Self {
        Self::Init(InitGameState::default())
    }
}

pub mod prelude {
    pub use super::*;

    pub use board_game_state::*;
    pub use greet_game_state::*;
    pub use init_game_state::*;
    pub use overview_game_state::*;
    pub use question_answer_game_state::*;
    pub use question_appearance_game_state::*;
    pub use question_asking_game_state::*;
    pub use question_matter_game_state::*;
    pub use question_qua_answering_game_state::*;
    pub use question_qua_queue_game_state::*;
    pub use question_qua_waiting_game_state::*;
}
