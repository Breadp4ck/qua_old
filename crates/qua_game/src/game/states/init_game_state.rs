use super::*;

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct InitGameState;

impl GameStateInteraction for InitGameState {
    fn handle_event(
        &mut self,
        context: &mut GameContext,
        event: &InputEvent,
        author: &PersonName,
        persons: &mut Persons,
        _: &mut PackageState,
    ) -> Option<GameState> {
        let person = persons.get(author).unwrap();
        match (event, person) {
            (InputEvent::Timeout, Person::Host(_)) => {
                context
                    .events
                    .push(GameEvent::Board(BoardUpdate::Text("ВСЕМ ПРИВЕТ".into())));

                Some(GameState::Greet(GreetGameState::default()))
            }
            _ => None,
        }
    }
}
