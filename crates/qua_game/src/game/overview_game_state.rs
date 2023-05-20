use super::*;

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct OverviewGameState;

impl GameStateInteraction for OverviewGameState {
    fn handle_event(
        &mut self,
        _: &mut GameContext,
        event: &InputEvent,
        _: &mut Person,
    ) -> Option<GameState> {
        match event {
            InputEvent::Timeout => Some(GameState::Board(BoardGameState::default())),
            _ => None,
        }
    }
}
