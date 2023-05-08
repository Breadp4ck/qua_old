use std::time::Duration;

use super::{
    asking_game_state::AskingGameState, prelude::PersonName, EmiterType, GameAction, GameEvent,
    GameState, Question, show_answer_game_state::ShowAnswerGameState,
};

pub struct AnsweringGameState {
    player_name: PersonName,
    question: Question,
    time: Duration,
    asking_elapsed_time: Duration,
}

impl AnsweringGameState {
    pub fn with_player(player_name: PersonName, question: Question, asking_elapsed_time: Duration) -> Self {
        Self {
            player_name,
            question,
            time: Duration::from_secs(0),
            asking_elapsed_time,
        }
    }
}

impl GameState for AnsweringGameState {
    fn tick(&mut self, delta: Duration) -> (Option<Box<dyn GameState>>, Option<GameEvent>) {
        self.time += delta;

        if self.time > Duration::from_secs(10) {
            return (
                Some(Box::new(AskingGameState::with_question_elapsed(
                    self.question,
                    self.asking_elapsed_time,
                ))),
                None,
            );
        }

        (None, None)
    }

    fn handle(
        &mut self,
        emiter: EmiterType,
        action: GameAction,
    ) -> (Option<Box<dyn GameState>>, Option<GameEvent>) {
        if emiter == EmiterType::Host {
            if let GameAction::ScoreAnswer { correct } = action {
                if correct {
                    return (
                        Some(Box::new(ShowAnswerGameState::of_question(self.question))),
                        None,
                    );
                } else {
                    return (
                        Some(Box::new(AskingGameState::with_question_elapsed(
                            self.question,
                            self.asking_elapsed_time,
                        ))),
                        None,
                    );
                }
            }
        }

        (None, None)
    }
}
