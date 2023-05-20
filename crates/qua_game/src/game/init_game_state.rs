use super::*;

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct InitGameState;

impl GameStateInteraction for InitGameState {
    fn handle_event(
        &mut self,
        _: &mut GameContext,
        event: &InputEvent,
        author: &mut Person,
    ) -> Option<GameState> {
        match (event, author) {
            (InputEvent::Begin, Person::Host(_)) => {
                Some(GameState::Greet(GreetGameState::default()))
            }
            _ => None,
        }
    }
}
