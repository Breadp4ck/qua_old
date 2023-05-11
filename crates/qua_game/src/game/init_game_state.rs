use super::*;

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct InitGameState;

impl GameStateInteraction for InitGameState {
    fn handle_event(
        &mut self,
        context: &mut GameContext,
        event: &StateInputEvent,
        author: &mut Person,
    ) -> Option<GameState> {
        match (event, author) {
            (StateInputEvent::Begin, Person::Host(_)) => {
                Some(GameState::Greet(GreetGameState::default()))
            }
            _ => None,
        }
    }
}
