use std::io::{Write, Read};
use std::{collections::HashMap, fs::File, future::Future, sync::Arc};

use tokio::sync::Mutex;

use crate::{models::prelude::*, UPLOADS_DIRECTORY};

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

    pub fn create_room(&self, file_data: &[u8]) -> impl Future<Output = (RoomId, RoomCode)> {
        let shared = self.0.clone();
        let file_data = file_data.to_vec();
        async move { shared.lock().await.create_room(file_data) }
    }

    pub fn get_package(&self, room_code: RoomCode) -> impl Future<Output = Vec<u8>> {
        let shared = self.0.clone();
        async move { shared.lock().await.get_package(&room_code) }
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

    fn create_room(&mut self, file_data: Vec<u8>) -> (RoomId, RoomCode) {
        let room_code = self.generate_room_code();
        let room_id = self.last_room_id;

        self.rooms.insert(room_code.clone(), room_id);
        self.last_room_id.next();

        let mut file = File::create(format!("{}/{}_game.qua", UPLOADS_DIRECTORY, room_id))
            .expect("Failed to create file");

        file.write_all(&file_data).expect("Failed to write to file");

        (room_id, room_code)
    }

    fn get_package(&self, room_code: &RoomCode) -> Vec<u8> {
        let room_id = self.get_room_id(room_code).unwrap();

        let mut file = File::open(format!("{}/{}_game.qua", UPLOADS_DIRECTORY, room_id))
            .expect("Failed to open file");

        let mut file_data: Vec<u8> = vec![];
        file.read_to_end(&mut file_data).expect("Failed to read file");

        file_data
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
