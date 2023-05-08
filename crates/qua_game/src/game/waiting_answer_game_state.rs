use std::time::Duration;

use super::{GameEvent, GameState, GameAction, EmiterType};

#[derive(Default)]
pub struct WaitingAnswerGameState;

impl GameState for WaitingAnswerGameState {
    fn tick(&mut self, _: Duration) -> (Option<Box<dyn GameState>>, Option<GameEvent>) {
        (None, None)
    }

    fn handle(
        &mut self,
        emiter: EmiterType,
        action: GameAction,
    ) -> (Option<Box<dyn GameState>>, Option<GameEvent>) {
        (None, None)
    }
}
