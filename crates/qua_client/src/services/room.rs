use log::*;
use std::ops::Deref;

use ewebsock::{WsMessage, WsReceiver, WsSender};
use qua_game::{game::ClientMessage, person::Person};
use serde::{Deserialize, Serialize};
use wasm_sockets::{self, PollingClient, WebSocketError};

#[derive(Serialize, Deserialize)]
pub struct CreateRoomRequest {
    name: String,
}

#[derive(Serialize, Deserialize)]
pub struct TicketData {
    code: RoomCode,
    person: Person,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoomCode {
    pub code: String, //todo remove pub
}

impl ToString for RoomCode {
    fn to_string(&self) -> String {
        self.code.clone()
    }
}

impl From<String> for RoomCode {
    fn from(code: String) -> Self {
        Self { code }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Ticket {
    id: String,
}

impl ToString for Ticket {
    fn to_string(&self) -> String {
        self.id.clone()
    }
}

pub struct RoomService;

impl RoomService {
    pub async fn create_room() -> RoomCode {
        let room_code: RoomCode = reqwest::Client::new()
            .get("http://localhost:8000/api/room/create")
            // .json(&request)
            .send()
            .await
            .expect("Failed send request")
            .json::<RoomCode>()
            .await
            .expect("Failed to obtain room code");

        info!("Obtained room code: {:#?}", room_code);

        room_code
    }

    pub fn join_room(ticket: &Ticket) -> PollingClient {
        let ticket = ticket.to_string();

        let client = PollingClient::new(&format!("ws://localhost:8000/api/room/join/{ticket}"))
            .expect("Failed to connect");

        client
    }

    pub async fn obtain_ticket(person: Person, code: RoomCode) -> Ticket {
        let request = TicketData { person, code };

        let ticket: Ticket = reqwest::Client::new()
            .post("http://localhost:8000/api/room/obtain_ticket")
            .json(&request)
            .send()
            .await
            .expect("Failed send request")
            .json::<Ticket>()
            .await
            .expect("Failed to obtain room ticket");

        info!("Obtained ticket: {:#?}", ticket);

        ticket
    }
}
