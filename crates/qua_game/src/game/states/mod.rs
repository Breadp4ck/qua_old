use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::*;

mod greet_game_state;
mod init_game_state;
mod overview_game_state;
mod picking_game_state;
mod question_answer_game_state;
mod question_appearance_game_state;
mod question_asking_game_state;
mod question_matter_game_state;
mod question_qua_answering_game_state;
mod question_qua_queue_game_state;
mod question_qua_waiting_game_state;

pub use greet_game_state::*;
pub use init_game_state::*;
pub use overview_game_state::*;
pub use picking_game_state::*;
pub use question_answer_game_state::*;
pub use question_appearance_game_state::*;
pub use question_asking_game_state::*;
pub use question_matter_game_state::*;
pub use question_qua_answering_game_state::*;
pub use question_qua_queue_game_state::*;
pub use question_qua_waiting_game_state::*;

type Persons = HashMap<PersonName, Person>;

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
    Picking(PickingGameState),
    QuestionAppearance(QuestionAppearanceGameState),
    QuestionMatter(QuestionMatterGameState),
    QuestionAsking(QuestionAskingGameState),
    QuestionQuaWaiting(QuestionQuaWaitingGameState),
    QuestionQuaQueue(QuestionQuaQueueGameState),
    QuestionQuaAnswering(QuestionQuaAnsweringGameState),
    QuestionAnswer(QuestionAnswerGameState),
}

impl GameState {
    pub fn proceed_translation_event(&mut self, context: &mut GameContext) {
        match self {
            GameState::Init(_) => context.events.push(GameEvent::State(StateUpdate::Init)),
            GameState::Greet(_) => context.events.push(GameEvent::State(StateUpdate::Greet)),
            GameState::Overview(_) => context.events.push(GameEvent::State(StateUpdate::Overview)),
            GameState::Picking(_) => context.events.push(GameEvent::State(StateUpdate::Picking)),
            GameState::QuestionAppearance(_) => context.events.push(GameEvent::State(StateUpdate::QuestionAppearance)),
            GameState::QuestionMatter(_) => context.events.push(GameEvent::State(StateUpdate::QuestionMatter)),
            GameState::QuestionAsking(_) => context.events.push(GameEvent::State(StateUpdate::QuestionAsking)),
            GameState::QuestionQuaWaiting(_) => context.events.push(GameEvent::State(StateUpdate::QuaWaiting)),
            GameState::QuestionQuaQueue(_) => context.events.push(GameEvent::State(StateUpdate::QuaQueue)),
            GameState::QuestionQuaAnswering(_) => context.events.push(GameEvent::State(StateUpdate::QuaAnswer)),
            GameState::QuestionAnswer(_) => context.events.push(GameEvent::State(StateUpdate::QuestionAnswer)),
        }
    }

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
            GameState::Overview(state) => {
                state.handle_event(context, event, author, persons, package)
            }
            GameState::Picking(state) => {
                state.handle_event(context, event, author, persons, package)
            }
            GameState::QuestionAppearance(state) => {
                state.handle_event(context, event, author, persons, package)
            }
            GameState::QuestionMatter(state) => {
                state.handle_event(context, event, author, persons, package)
            }
            GameState::QuestionAsking(state) => {
                state.handle_event(context, event, author, persons, package)
            }
            GameState::QuestionQuaWaiting(state) => {
                state.handle_event(context, event, author, persons, package)
            }
            GameState::QuestionQuaQueue(state) => {
                state.handle_event(context, event, author, persons, package)
            }
            GameState::QuestionQuaAnswering(state) => {
                state.handle_event(context, event, author, persons, package)
            }
            GameState::QuestionAnswer(state) => {
                state.handle_event(context, event, author, persons, package)
            }
        }
    }
}

impl Default for GameState {
    fn default() -> Self {
        Self::Init(InitGameState::default())
    }
}
