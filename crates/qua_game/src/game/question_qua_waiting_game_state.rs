use super::*;

#[derive(Default)]
pub struct QuestionQuaWaitingGameState;

impl GameStateInteraction for QuestionQuaWaitingGameState {
    fn handle_event(
        &mut self,
        context: &mut GameContext,
        event: &GameEventLocal,
        author: &mut Person,
    ) -> Option<GameState> {
        None
    }
}
