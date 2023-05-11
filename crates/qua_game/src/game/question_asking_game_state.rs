use super::*;

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct QuestionAskingGameState;

impl GameStateInteraction for QuestionAskingGameState {
    fn handle_event(
        &mut self,
        context: &mut GameContext,
        event: &StateInputEvent,
        author: &mut Person,
    ) -> Option<GameState> {
        match event {
            StateInputEvent::Timeout => Some(GameState::Overview(OverviewGameState::default())),
            _ => None,
        }
    }
}
