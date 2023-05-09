use super::*;

#[derive(Default)]
pub struct OverviewGameState;

impl GameStateInteraction for OverviewGameState {
    fn handle_event(
        &mut self,
        context: &mut GameContext,
        event: &GameEventLocal,
        author: &mut Person,
    ) -> Option<GameState> {
        match event {
            GameEventLocal::Timeout => Some(GameState::Board(BoardGameState::default())),
            _ => None,
        }
    }
}
