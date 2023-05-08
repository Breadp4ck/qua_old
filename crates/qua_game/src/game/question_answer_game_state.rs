use super::*;

#[derive(Default)]
pub struct QuestionAnswerGameState;

impl GameStateInteraction for QuestionAnswerGameState {
    fn handle_event(
        &mut self,
        context: &mut GameContext,
        event: GameEventLocal,
        author: &mut Person,
    ) -> Option<GameState> {
        None
    }
}
