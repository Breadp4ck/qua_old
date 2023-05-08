use qua_game::person::Person;
use serde::{Deserialize, Serialize};

use crate::RoomCode;

#[derive(Serialize, Deserialize, Clone)]
pub struct TicketData {
    code: RoomCode,
    person: Person,
}

impl TicketData {
    pub fn room_code(&self) -> RoomCode {
        self.code.clone()
    }

    pub fn person(&self) -> Person {
        self.person.clone()
    }
}
