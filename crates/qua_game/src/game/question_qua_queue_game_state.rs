use super::*;

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct QuestionQuaQueueGameState;

impl GameStateInteraction for QuestionQuaQueueGameState {
    fn handle_event(
        &mut self,
        context: &mut GameContext,
        event: &StateInputEvent,
        author: &mut Person,
    ) -> Option<GameState> {
        None
    }
}
