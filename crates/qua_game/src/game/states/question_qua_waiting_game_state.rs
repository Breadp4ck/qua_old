use super::*;

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct QuestionQuaWaitingGameState;

impl GameStateInteraction for QuestionQuaWaitingGameState {
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
                if let Some(question) = context.question {
                    package.mark_answered(&question);
                    context.question = None;
                }

                context
                    .events
                    .push(GameEvent::Board(BoardUpdate::AnswerMedia(
                        context.question.unwrap().clone(),
                    )));

                Some(GameState::QuestionAnswer(QuestionAnswerGameState::default()))
            }
            (InputEvent::Answer(delta), Person::Player(_)) => Some(GameState::QuestionQuaQueue(
                QuestionQuaQueueGameState::new(author, &delta),
            )),
            _ => None,
        }
    }
}
