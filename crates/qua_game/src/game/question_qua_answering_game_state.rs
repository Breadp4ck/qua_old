use super::*;

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct QuestionQuaAnsweringGameState;

impl GameStateInteraction for QuestionQuaAnsweringGameState {
    fn handle_event(
        &mut self,
        _: &mut GameContext,
        _: &InputEvent,
        _: &mut Person,
    ) -> Option<GameState> {
        None
    }
}
