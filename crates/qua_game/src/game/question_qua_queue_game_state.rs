use super::*;

#[derive(Default)]
pub struct QuestionQuaQueueGameState;

impl GameStateInteraction for QuestionQuaQueueGameState {
    fn handle_event(
        &mut self,
        context: &mut GameContext,
        event: &GameEventLocal,
        author: &mut Person,
    ) -> Option<GameState> {
        None
    }
}
