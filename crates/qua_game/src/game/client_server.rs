use serde::{Serialize, Deserialize};

use super::*;

#[derive(Serialize, Deserialize)]
pub enum ClientMessage {
    Input(InputEvent),
    Kick(PersonName),
    StatelessInput(StatelessInputEvent),
    SyncRequest,
}

#[derive(Serialize, Deserialize)]
pub enum ServerMessage {
    Input(InputEvent, PersonName),
    StatelessInput(StatelessInputEvent, PersonName),
    SyncResponse(Game),
    PersonConnected(Person),
    PersonDisconnected(PersonName),
}

