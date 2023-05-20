use super::*;

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct GreetGameState;

impl GameStateInteraction for GreetGameState {
    fn handle_event(
        &mut self,
        _: &mut GameContext,
        event: &InputEvent,
        _: &mut Person,
    ) -> Option<GameState> {
        match event {
            InputEvent::Timeout => Some(GameState::Overview(OverviewGameState::default())),
            _ => None,
        }
    }
}
