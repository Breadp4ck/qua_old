use super::*;

#[derive(Default)]
pub struct QuestionQuaAnsweringGameState;

impl GameStateInteraction for QuestionQuaAnsweringGameState {
    fn handle_event(
        &mut self,
        context: &mut GameContext,
        event: GameEventLocal,
        author: &mut Person,
    ) -> Option<GameState> {
        None
    }
}
