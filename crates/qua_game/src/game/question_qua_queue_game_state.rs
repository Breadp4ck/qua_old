use super::*;

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct QuestionQuaQueueGameState;

impl GameStateInteraction for QuestionQuaQueueGameState {
    fn handle_event(
        &mut self,
        _: &mut GameContext,
        _: &InputEvent,
        _: &mut Person,
    ) -> Option<GameState> {
        None
    }
}
