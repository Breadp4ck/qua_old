use super::*;

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct GreetGameState;

impl GameStateInteraction for GreetGameState {
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
                    .push(GameEvent::Board(BoardUpdate::Overview));

                Some(GameState::Overview(OverviewGameState::default()))
            }
            _ => None,
        }
    }
}
