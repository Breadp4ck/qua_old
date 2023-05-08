use std::time::Duration;

use super::{GameEvent, GameState, GameAction, EmiterType, overview_game_state::OverviewGameState, Question};

pub struct ShowAnswerGameState {
    question: Question,
    time: Duration,
}

impl ShowAnswerGameState {
    pub fn of_question(question: Question) -> Self {
        Self {
            question,
            time: Duration::from_secs(0),
        }
    }
}

impl GameState for ShowAnswerGameState {
    fn tick(&mut self, delta: Duration) -> (Option<Box<dyn GameState>>, Option<GameEvent>) {
        self.time += delta;

        if self.time > Duration::from_secs(5) {
            return (
                Some(Box::new(OverviewGameState::default())),
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
        (None, None)
    }
}
