use super::*;

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct QuestionQuaWaitingGameState;

impl GameStateInteraction for QuestionQuaWaitingGameState {
    fn handle_event(
        &mut self,
        _: &mut GameContext,
        _: &InputEvent,
        _: &mut Person,
    ) -> Option<GameState> {
        None
    }
}
