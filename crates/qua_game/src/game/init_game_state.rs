use std::time::Duration;

use super::{overview_game_state::OverviewGameState, EmiterType, GameAction, GameEvent, GameState};

#[derive(Default)]
pub struct InitGameState;

impl GameState for InitGameState {
    fn tick(&mut self, _: Duration) -> (Option<Box<dyn GameState>>, Option<GameEvent>) {
        (None, None)
    }

    fn handle(
        &mut self,
        emiter: EmiterType,
        action: GameAction,
    ) -> (Option<Box<dyn GameState>>, Option<GameEvent>) {
        if emiter == EmiterType::Host {
            if let GameAction::Begin { .. } = action {
                return (Some(Box::new(OverviewGameState::default())), None);
            }
        }

        (None, None)
    }
}
