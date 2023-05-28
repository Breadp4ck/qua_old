use super::*;

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct QuestionAnswerGameState;

impl GameStateInteraction for QuestionAnswerGameState {
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
                context.events.push(GameEvent::Board(BoardUpdate::Picking(
                    context.round.clone(),
                )));

                Some(GameState::Picking(PickingGameState::default()))
            }
            _ => None,
        }
    }
}
