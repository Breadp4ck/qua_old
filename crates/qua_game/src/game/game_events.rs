use serde::{Deserialize, Serialize};

use super::*;
use crate::scores::*;

#[derive(Clone, Serialize, Deserialize)]
pub enum GameEvent {
    Board(BoardUpdate),
    Player(PlayerUpdate),
    Host(HostUpdate),
    State(StateUpdate),
}

#[derive(Clone, Serialize, Deserialize)]
pub enum BoardUpdate {
    Init,
    Greet,
    Overview,
    RoundPreview(Round),
    QuestionType(Question),
    QuestionMatter(Question),
    QuestionMedia(Question),
    AnswerMedia(Question),
    Picking(Round),
    Ending,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum ScoreChange {
    Add { amount: Scores },
    Remove { amount: Scores },
    Set { from: Scores },
}

#[derive(Clone, Serialize, Deserialize)]
pub enum PlayerUpdate {
    Connected {
        name: PersonName,
    },
    Disconnected {
        name: PersonName,
    },
    BecomeLead {
        name: PersonName,
    },
    ScoresChanges {
        name: PersonName,
        change: ScoreChange,
        new_scores: Scores,
    },
    Sync,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum HostUpdate {
    Connected { name: PersonName },
    Disconnected,
    Sync,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum StateUpdate {
    Init,
    Greet,
    Overview,
    RoundPreview,
    Picking,
    QuestionAppearance,
    QuestionMatter,
    QuestionAsking,
    QuaWaiting,
    QuaQueue,
    QuaAnswer,
    QuestionAnswer,
    Ending,
}
