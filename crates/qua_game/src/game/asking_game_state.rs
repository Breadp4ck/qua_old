use std::time::Duration;

use super::{
    answering_game_state::AnsweringGameState, overview_game_state::OverviewGameState, EmiterType,
    GameAction, GameEvent, GameState, Question,
};

pub struct AskingGameState {
    question: Question,
    time: Duration,
}

impl AskingGameState {
    pub fn with_question(question: Question) -> Self {
        Self {
            question,
            time: Duration::from_secs(0),
        }
    }

    pub fn with_question_elapsed(question: Question, elapsed: Duration) -> Self {
        Self {
            question,
            time: elapsed,
        }
    }
}

impl GameState for AskingGameState {
    fn tick(&mut self, delta: Duration) -> (Option<Box<dyn GameState>>, Option<GameEvent>) {
        self.time += delta;

        if self.time > Duration::from_secs(10) {
            return (Some(Box::new(OverviewGameState::default())), None);
        }

        (None, None)
    }

    fn handle(
        &mut self,
        emiter: EmiterType,
        action: GameAction,
    ) -> (Option<Box<dyn GameState>>, Option<GameEvent>) {
        if emiter != EmiterType::Host {
            if let GameAction::Answer { player_name } = action {
                return (
                    Some(Box::new(AnsweringGameState::with_player(player_name, self.question, self.time))),
                    None,
                );
            }
        }
        (None, None)
    }
}
