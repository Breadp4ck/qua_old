use super::*;

#[derive(Clone, Serialize, Deserialize)]
pub struct QuestionQuaAnsweringGameState {
    answering: PersonName,
}

impl QuestionQuaAnsweringGameState {
    pub fn new(answering: PersonName) -> Self {
        Self { answering }
    }
}

impl GameStateInteraction for QuestionQuaAnsweringGameState {
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
            (InputEvent::CountCorrect, Person::Host(_)) => {
                let scores = package.get(&context.question.unwrap()).cost;
                if let Person::Player(player) = persons.get_mut(&self.answering).unwrap() {
                    player.add_scores(scores);
                }

                Some(GameState::QuestionAnswer(QuestionAnswerGameState::default()))
            }
            (InputEvent::CountWrong | InputEvent::Timeout, Person::Host(_)) => {
                let scores = package.get(&context.question.unwrap()).cost;
                if let Person::Player(player) = persons.get_mut(&self.answering).unwrap() {
                    player.remove_scores(scores);
                }

                Some(GameState::QuestionQuaWaiting(
                    QuestionQuaWaitingGameState::default(),
                ))
            }
            _ => None,
        }
    }
}
