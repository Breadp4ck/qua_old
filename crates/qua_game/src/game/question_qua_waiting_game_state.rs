use super::*;

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct QuestionQuaWaitingGameState;

impl GameStateInteraction for QuestionQuaWaitingGameState {
    fn handle_event(
        &mut self,
        context: &mut GameContext,
        event: &StateInputEvent,
        author: &mut Person,
    ) -> Option<GameState> {
        None
    }
}
