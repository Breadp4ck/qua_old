use super::*;

#[derive(Default)]
pub struct QuestionMatterGameState;

impl GameStateInteraction for QuestionMatterGameState {
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
