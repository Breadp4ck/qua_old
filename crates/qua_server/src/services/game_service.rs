use axum::extract::ws::WebSocket;
use std::{collections::HashMap, sync::Arc, future::Future};
use tokio::sync::{
    mpsc::{self, UnboundedSender},
    Mutex,
};

use crate::models::prelude::*;

pub enum GameConnectionEvent {
    Connect(WebSocket, TicketData, RoomId),
}

#[derive(Clone)]
pub struct GameService(Arc<Mutex<GameServiceInner>>);

impl GameService {
    pub fn new() -> Self {
        Self(Arc::new(Mutex::new(GameServiceInner::default())))
    }

    pub async fn handle_socket(
        sender: UnboundedSender<GameConnectionEvent>,
        socket: WebSocket,
        ticket: TicketData,
        room_id: RoomId,
    ) {
        if let Err(_) = sender.send(GameConnectionEvent::Connect(socket, ticket, room_id)) {
            // TODO: log error
        };
    }

    pub fn create_room(&self, room_id: RoomId) -> impl Future<Output = ()> {
        let shared = self.0.clone();

        async move {
            shared.lock().await.create_room(room_id)
        }
    }

    pub fn event_sender(&self) -> UnboundedSender<GameConnectionEvent> {
        let (sender, mut receiver) = mpsc::unbounded_channel::<GameConnectionEvent>();
        let shared = self.0.clone();

        tokio::spawn(async move {
            while let Some(event) = receiver.recv().await {
                let GameConnectionEvent::Connect(socket, ticket, room_id) = event;
                shared.lock().await.connect(socket, ticket, room_id);
            }
        });

        sender
    }
}

struct GameServiceInner {
    rooms: HashMap<RoomId, Room>,
}

impl GameServiceInner {
    fn connect(&mut self, socket: WebSocket, ticket_data: TicketData, room_id: RoomId) {
        let room = if let Some(room) = self.rooms.get_mut(&room_id) {
            room
        } else {
            // TODO: make normal handler
            panic!("Room not found");
        };

        room.add_person(socket, ticket_data.person());
    }

    pub fn create_room(&mut self, room_id: RoomId) {
        self.rooms.insert(room_id, Room::new(qua_game::package::prelude::Package::default()));
    }
}

impl Default for GameServiceInner {
    fn default() -> Self {
        Self {
            rooms: HashMap::new(),
        }
    }
}
