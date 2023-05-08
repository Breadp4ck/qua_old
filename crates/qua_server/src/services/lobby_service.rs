use std::{collections::HashMap, future::Future, sync::Arc};

use tokio::sync::Mutex;

use crate::models::prelude::*;

#[derive(Clone)]
pub struct LobbyService(Arc<Mutex<LobbyServiceInner>>);

impl LobbyService {
    pub fn new() -> Self {
        Self(Arc::new(Mutex::new(LobbyServiceInner::new())))
    }

    pub fn get_room_id(&self, room_code: RoomCode) -> impl Future<Output = Option<RoomId>> {
        let shared = self.0.clone();
        async move { shared.lock().await.get_room_id(&room_code) }
    }

    pub fn create_room(&self) -> impl Future<Output = (RoomId, RoomCode)> {
        let shared = self.0.clone();
        async move { shared.lock().await.create_room() }
    }
}

struct LobbyServiceInner {
    rooms: HashMap<RoomCode, RoomId>,
    last_room_id: RoomId,
}

impl LobbyServiceInner {
    fn new() -> Self {
        Self::default()
    }

    fn get_room_id(&self, room_code: &RoomCode) -> Option<RoomId> {
        self.rooms.get(room_code).cloned()
    }

    fn create_room(&mut self) -> (RoomId, RoomCode) {
        let room_code = self.generate_room_code();
        let room_id = self.last_room_id;

        self.rooms.insert(room_code.clone(), room_id);
        self.last_room_id.next();

        (room_id, room_code)
    }

    fn generate_room_code(&self) -> RoomCode {
        let mut room_code = RoomCode::random();

        while let true = self.rooms.contains_key(&room_code) {
            room_code = RoomCode::random();
        }

        room_code
    }
}

impl Default for LobbyServiceInner {
    fn default() -> Self {
        Self {
            rooms: HashMap::new(),
            last_room_id: RoomId::min(),
        }
    }
}
