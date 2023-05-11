use super::*;

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct OverviewGameState;

impl GameStateInteraction for OverviewGameState {
    fn handle_event(
        &mut self,
        context: &mut GameContext,
        event: &StateInputEvent,
        author: &mut Person,
    ) -> Option<GameState> {
        match event {
            StateInputEvent::Timeout => Some(GameState::Board(BoardGameState::default())),
            _ => None,
        }
    }
}
