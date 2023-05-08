use std::time::Duration;

use super::{asking_game_state::AskingGameState, EmiterType, GameAction, GameEvent, GameState};

#[derive(Default)]
pub struct OverviewGameState;

impl GameState for OverviewGameState {
    fn tick(&mut self, _: Duration) -> (Option<Box<dyn GameState>>, Option<GameEvent>) {
        (None, None)
    }

    fn handle(
        &mut self,
        emiter: EmiterType,
        action: GameAction,
    ) -> (Option<Box<dyn GameState>>, Option<GameEvent>) {
        if emiter == EmiterType::Host || emiter == EmiterType::Lead {
            if let GameAction::Select { question } = action {
                return (
                    Some(Box::new(AskingGameState::with_question(question))),
                    None,
                );
            }
        }
        (None, None)
    }
}
