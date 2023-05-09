use super::*;

#[derive(Default)]
pub struct InitGameState;

impl GameStateInteraction for InitGameState {
    fn handle_event(
        &mut self,
        context: &mut GameContext,
        event: &GameEventLocal,
        author: &mut Person,
    ) -> Option<GameState> {
        match (event, author) {
            (GameEventLocal::Begin, Person::Host(_)) => {
                Some(GameState::Greet(GreetGameState::default()))
            }
            _ => None,
        }
    }
}
