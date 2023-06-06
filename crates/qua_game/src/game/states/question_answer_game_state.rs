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
        package: &mut PackageState,
    ) -> Option<GameState> {
        let person = persons.get(author).unwrap();
        match (event, person) {
            (InputEvent::Timeout, Person::Host(_)) => {
                if package.is_clear(&context.round) {
                    if package.is_last(&context.round) {
                        context.events.push(GameEvent::Board(BoardUpdate::Ending));
                        Some(GameState::Ending(EndingGameState::default()))
                    } else {
                        if let Round::Normal(idx) = &mut context.round {
                            *idx += 1;
                        }
                        context.events.push(GameEvent::Board(BoardUpdate::Picking(
                            context.round.clone(),
                        )));
                        Some(GameState::Picking(PickingGameState::default()))
                    }
                } else {
                    context.events.push(GameEvent::Board(BoardUpdate::Picking(
                        context.round.clone(),
                    )));
                    Some(GameState::Picking(PickingGameState::default()))
                }
            }
            _ => None,
        }
    }
}
