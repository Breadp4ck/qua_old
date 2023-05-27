use super::*;

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct PickingGameState;

impl GameStateInteraction for PickingGameState {
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
            (InputEvent::SelectQuestion(question), Person::Player(player)) => {
                if let Some(leader_name) = context.lead.clone() {
                    let question = question.clone();
                    let current_round = context.round.clone();

                    match (question, current_round) {
                        (Question::Final(_), Round::Final) => todo!(),
                        (
                            Question::Normal(round_index, _, _),
                            Round::Normal(current_round_index),
                        ) => {
                            if *player.name() == leader_name
                                && round_index.clone() == current_round_index
                            {
                                context
                                    .events
                                    .push(GameEvent::Board(BoardUpdate::QuestionType(
                                        question.clone(),
                                    )));
                                context.question = Some(question.clone());
                                return Some(GameState::QuestionAppearance(
                                    QuestionAppearanceGameState::default(),
                                ));
                            }
                        }
                        _ => (),
                    }
                }

                None
            }
            (InputEvent::SelectQuestion(question), Person::Host(_)) => {
                let question = question.clone();
                let current_round = context.round.clone();

                match (question, current_round) {
                    (Question::Final(_), Round::Final) => todo!(),
                    (Question::Normal(round_index, _, _), Round::Normal(current_round_index)) => {
                        if round_index.clone() == current_round_index {
                            context
                                .events
                                .push(GameEvent::Board(BoardUpdate::QuestionType(
                                    question.clone(),
                                )));
                            context.question = Some(question.clone());
                            return Some(GameState::QuestionAppearance(
                                QuestionAppearanceGameState::default(),
                            ));
                        } else {
                            None
                        }
                    }
                    _ => None,
                }
            }
            _ => None,
        }
    }
}
