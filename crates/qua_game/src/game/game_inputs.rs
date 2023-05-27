use serde::{Serialize, Deserialize};
use std::time::Duration;

use super::*;
use crate::scores::*;

#[derive(Serialize, Deserialize)]
pub enum InputEvent {
    Begin,
    SelectQuestion(Question),
    Answer(Duration),
    CountCorrect,
    CountWrong,
    Skip,
    Timeout,
}

#[derive(Serialize, Deserialize)]
pub enum StatelessInputEvent {
    AssignLeadPlayer(PersonName),
    GivePlayerScores(PersonName, Scores),
    TakePlayerScores(PersonName, Scores),
    SetPlayerScores(PersonName, Scores),
}

