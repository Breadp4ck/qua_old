use ewebsock::{WsReceiver, WsSender};
use qua_game::person::Person;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CreateRoomRequest {
    name: String,
}

#[derive(Serialize, Deserialize)]
pub struct TicketData {
    code: RoomCode,
    person: Person,
}

#[derive(Debug, Serialize, Deserialize)]
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
    pub async fn create_room(_: String) -> RoomCode {
        let room_code: RoomCode = reqwest::Client::new()
            .get("http://localhost:8000/api/room/create")
            // .json(&request)
            .send()
            .await
            .expect("Failed send request")
            .json::<RoomCode>()
            .await
            .expect("Failed to obtain room code");

        room_code
    }

    pub async fn join_room(ticket: Ticket) -> (WsSender, WsReceiver) {
        let ticket = ticket.to_string();

        let (sender, receiver) =
            ewebsock::connect(format!("ws://localhost:8000/api/room/join/{ticket}"))
                .expect("Failed to connect");

        (sender, receiver)
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

        ticket
    }
}
