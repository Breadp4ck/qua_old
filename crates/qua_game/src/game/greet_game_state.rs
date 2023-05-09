use super::*;

#[derive(Default)]
pub struct GreetGameState;

impl GameStateInteraction for GreetGameState {
    fn handle_event(
        &mut self,
        context: &mut GameContext,
        event: &GameEventLocal,
        author: &mut Person,
    ) -> Option<GameState> {
        match event {
            GameEventLocal::Timeout => Some(GameState::Overview(OverviewGameState::default())),
            _ => None,
        }
    }
}
