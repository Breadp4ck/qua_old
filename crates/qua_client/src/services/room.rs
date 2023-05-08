use qua_game::person::Person;
use serde::{Deserialize, Serialize};
use ws_stream_wasm::{WsMeta, WsStream};

#[derive(Serialize, Deserialize)]
pub struct CreateRoomRequest {
    name: String,
}

#[derive(Serialize, Deserialize)]
pub struct ObtainTicketRequest {
    code: RoomCode,
    person: Person,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoomCode {
    pub data: String, //todo remove pub
}

impl ToString for RoomCode {
    fn to_string(&self) -> String {
        self.data.clone()
    }
}

impl From<String> for RoomCode {
    fn from(data: String) -> Self {
        Self { data }
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
    pub async fn create_room(name: String) -> RoomCode {
        let request = CreateRoomRequest { name };

        let room_code: RoomCode = reqwest::Client::new()
            .post("http://localhost:8000/api/room/create")
            .json(&request)
            .send()
            .await
            .expect("Failed send request")
            .json::<RoomCode>()
            .await
            .expect("Failed to obtain room code");

        room_code
    }

    pub async fn join_room(ticket: Ticket) -> (WsMeta, WsStream) {
        let ticket = ticket.to_string();

        let (ws, wsio) = ws_stream_wasm::WsMeta::connect(
            format!("ws://localhost:8000/api/room/join/{ticket}"),
            None,
        )
        .await
        .expect("assume the connection succeeds");

        (ws, wsio)
    }

    pub async fn obtain_ticket(person: Person, code: RoomCode) -> Ticket {
        let request = ObtainTicketRequest { person, code };

        let ticket: Ticket = reqwest::Client::new()
            .post("http://localhost:8000/api/room/obtain_ticket")
            .json(&request)
            .send()
            .await
            .expect("Failed send request")
            .json::<Ticket>()
            .await
            .expect("Failed to obtain room ticket");

        ticket
    }
}
