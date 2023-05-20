use super::*;

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct QuestionAnswerGameState;

impl GameStateInteraction for QuestionAnswerGameState {
    fn handle_event(
        &mut self,
        _: &mut GameContext,
        _: &InputEvent,
        _: &mut Person,
    ) -> Option<GameState> {
        None
    }
}
