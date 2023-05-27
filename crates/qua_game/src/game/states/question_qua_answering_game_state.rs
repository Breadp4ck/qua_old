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
                    context.lead = Some(self.answering.clone());

                    context
                        .events
                        .push(GameEvent::Player(PlayerUpdate::ScoresChanges {
                            name: player.name().clone(),
                            change: ScoreChange::Add { amount: scores },
                            new_scores: player.scores(),
                        }));
                    context
                        .events
                        .push(GameEvent::Player(PlayerUpdate::BecomeLead {
                            name: player.name().clone(),
                        }));
                }

                context
                    .events
                    .push(GameEvent::Board(BoardUpdate::AnswerMedia(
                        context.question.unwrap().clone(),
                    )));

                Some(GameState::QuestionAnswer(QuestionAnswerGameState::default()))
            }
            (InputEvent::CountWrong | InputEvent::Timeout, Person::Host(_)) => {
                let scores = package.get(&context.question.unwrap()).cost;
                if let Person::Player(player) = persons.get_mut(&self.answering).unwrap() {
                    player.remove_scores(scores);
                    context
                        .events
                        .push(GameEvent::Player(PlayerUpdate::ScoresChanges {
                            name: player.name().clone(),
                            change: ScoreChange::Remove { amount: scores },
                            new_scores: player.scores(),
                        }));
                }

                Some(GameState::QuestionQuaWaiting(
                    QuestionQuaWaitingGameState::default(),
                ))
            }
            _ => None,
        }
    }
}
