use std::collections::HashMap;
use std::time::{Duration, Instant};

use crate::package::prelude::*;
use crate::person::prelude::*;
use crate::person::*;
use crate::scores::Scores;

mod answering_game_state;
mod asking_game_state;
mod init_game_state;
mod overview_game_state;
mod show_answer_game_state;
mod waiting_answer_game_state;

// pub enum BoardState {
//     RoundOverview,
//     QuestionAsking,
//     QuestionWaiting,
//     QuestionAnswering(Option<PlayerName>),
//     QuestionShowingAnswer,
// }

#[derive(PartialEq)]
pub enum EmiterType {
    Host,
    Lead,
    Player,
}

pub enum Round {
    Default(RoundIndex),
    Final,
}

#[derive(Clone, Copy)]
pub enum Question {
    Final(QuestionIndex),
    Normal(RoundIndex, ThemeIndex, QuestionIndex),
}

pub trait GameState {
    fn tick(&mut self, delta: Duration) -> (Option<Box<dyn GameState>>, Option<GameEvent>);
    fn handle(
        &mut self,
        emiter: EmiterType,
        action: GameAction,
    ) -> (Option<Box<dyn GameState>>, Option<GameEvent>);
}

pub enum GameAction {
    Begin {},
    Select { question: Question },
    Answer { player_name: PersonName },
    ScoreAnswer { correct: bool },
}

pub enum GameEvent {
    CountCorrectAnswer { to: PersonName, amount: Scores },
    CountWrongAnswer { to: PersonName, amount: Scores },
    WaitAnswerTimeout { question: Question },
}

pub enum HostAction {
    MakeLead { who: PersonName },
    AddScores { to: PersonName, amount: Scores },
    RemoveScores { to: PersonName, amount: Scores },
    SetScores { to: PersonName, amount: Scores },
    Kick { who: PersonName },
}

pub struct Game {
    package: Package,
    round: Round,
    // state: Box<dyn GameState>,
    question: Option<Question>,
    host: Option<Host>,
    lead_player: Option<PersonName>,
    players: HashMap<PersonName, Player>,
    previous_moment: Instant,
}

impl Game {
    pub fn new(package: Package) -> Self {
        Game {
            package,
            round: Round::Default(RoundIndex::first()),
            // state: Box::new(init_game_state::InitGameState::default()),
            question: None,
            host: None,
            lead_player: None,
            players: HashMap::new(),
            previous_moment: Instant::now(),
        }
    }

    pub fn add_host(&mut self, host: Host) {
        self.host = Some(host);
    }

    pub fn add_player(&mut self, player: Player) {
        self.players.insert(player.name().clone(), player);
    }

    pub fn kick_player(&mut self, name: PersonName) {
        self.players.remove(&name);
    }

    pub fn handle_action(&mut self, author: PersonName, action: GameAction) {
        // let mut emiter = EmiterType::Player;

        // if let Some(lead_player) = self.lead_player.take() {
        //     if lead_player == author {
        //         emiter = EmiterType::Lead;
        //     }
        // }

        // let (maybe_new_state, maybe_event) = self.state.handle(emiter, action);

        // if let Some(event) = maybe_event {
        //     self.handle_event(event);
        // }

        // if let Some(new_state) = maybe_new_state {
        //     self.state = new_state;
        // }
    }

    pub fn handle_event(&mut self, event: GameEvent) {
        match event {
            GameEvent::CountCorrectAnswer { to, amount } => {
                self.lead_player = Some(to.clone());
                self.players.get_mut(&to).unwrap().add_scores(amount)
            }
            GameEvent::CountWrongAnswer { to, amount } => {
                self.players.get_mut(&to).unwrap().remove_scores(amount)
            }
            GameEvent::WaitAnswerTimeout { question } => {
                self.package.mark_answered(&question);
            }
        }
    }

    pub fn handle_host_action(&mut self, action: HostAction) {
        match action {
            HostAction::AddScores { to, amount } => {
                self.players.get_mut(&to).unwrap().add_scores(amount)
            }
            HostAction::RemoveScores { to, amount } => {
                self.players.get_mut(&to).unwrap().remove_scores(amount)
            }
            HostAction::SetScores { to, amount } => {
                self.players.get_mut(&to).unwrap().set_scores(amount)
            }
            HostAction::MakeLead { who } => self.lead_player = Some(who.clone()),
            _ => (),
        }
    }
}

pub mod prelude {
    pub use super::*;
}
