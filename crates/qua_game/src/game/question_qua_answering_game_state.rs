use super::*;

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct QuestionQuaAnsweringGameState;

impl GameStateInteraction for QuestionQuaAnsweringGameState {
    fn handle_event(
        &mut self,
        context: &mut GameContext,
        event: &StateInputEvent,
        author: &mut Person,
    ) -> Option<GameState> {
        None
    }
}
