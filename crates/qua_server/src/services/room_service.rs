use std::io::{Cursor, Read, Write};
use std::{collections::HashMap, fs::File, future::Future, sync::Arc};

use axum::extract::ws::WebSocket;
use qua_game::package::prelude::Package;
use qua_package::package_config::PackageConfig;
use tokio::sync::mpsc::UnboundedSender;
use tokio::sync::{mpsc, Mutex};

use crate::{models::prelude::*, UPLOADS_DIRECTORY};

pub enum RoomServiceEvent {
    UserConnected(WebSocket, TicketData, RoomCode),
    RoomEmptied(RoomCode),
}

#[derive(Clone)]
pub struct RoomService {
    inner: Arc<Mutex<RoomServiceInner>>,
    sender: UnboundedSender<RoomServiceEvent>,
}

impl RoomService {
    pub fn new() -> Self {
        let (sender, mut receiver) = mpsc::unbounded_channel::<RoomServiceEvent>();
        let inner = Arc::new(Mutex::new(RoomServiceInner::new()));

        let shared = inner.clone();

        tokio::spawn(async move {
            while let Some(event) = receiver.recv().await {
                match event {
                    RoomServiceEvent::UserConnected(socket, ticket, room_code) => {
                        shared
                            .lock()
                            .await
                            .connect(socket, &ticket, &room_code)
                            .await;
                    }
                    RoomServiceEvent::RoomEmptied(room_code) => {
                        shared.lock().await.remove_room(&room_code)
                    }
                }
            }
        });

        Self { inner, sender }
    }

    pub async fn handle_socket(
        sender: UnboundedSender<RoomServiceEvent>,
        socket: WebSocket,
        ticket: TicketData,
        room_code: RoomCode,
    ) {
        if let Err(_) = sender.send(RoomServiceEvent::UserConnected(socket, ticket, room_code)) {
            log::error!("Failed to send user connected event");
        };
    }

    pub fn exists(&self, room_code: RoomCode) -> impl Future<Output = bool> {
        let shared = self.inner.clone();
        async move { shared.lock().await.exists(&room_code) }
    }

    pub fn create_room(&self, file_data: &[u8]) -> impl Future<Output = RoomCode> {
        let shared = self.inner.clone();
        let sender = self.sender.clone();
        let file_data = file_data.to_vec();
        async move { shared.lock().await.create_room(file_data, sender) }
    }

    pub fn get_package(&self, room_code: RoomCode) -> impl Future<Output = Vec<u8>> {
        let shared = self.inner.clone();
        async move { shared.lock().await.get_package(&room_code) }
    }

    pub fn sender(&self) -> UnboundedSender<RoomServiceEvent> {
        self.sender.clone()
    }
}

struct RoomServiceInner {
    rooms: HashMap<RoomCode, (RoomId, Room)>,
    last_room_id: RoomId,
}

impl RoomServiceInner {
    fn new() -> Self {
        Self::default()
    }

    async fn connect(&mut self, socket: WebSocket, ticket_data: &TicketData, room_code: &RoomCode) {
        let (_, room) = if let Some((id, room)) = self.rooms.get_mut(room_code) {
            (id, room)
        } else {
            // TODO: make normal handler
            panic!("Room not found");
        };

        room.add_person(socket, ticket_data.person()).await;
    }

    fn exists(&self, room_code: &RoomCode) -> bool {
        if let Some(_) = self.rooms.get(room_code) {
            true
        } else {
            false
        }
    }

    fn create_room(
        &mut self,
        file_data: Vec<u8>,
        sender: UnboundedSender<RoomServiceEvent>,
    ) -> RoomCode {
        let room_code = self.generate_room_code();
        let room_id = self.last_room_id;

        {
            // TODO: Exceptions in this block supposed to return error to client
            let mut file = File::create(format!("{}/{}_game.qua", UPLOADS_DIRECTORY, room_id))
                .expect("Failed to create file");

            file.write_all(&file_data).expect("Failed to write to file");
        }

        let mut zip = zip::ZipArchive::new(Cursor::new(file_data)).unwrap();

        let config = if let Ok(mut config) = zip.by_name("Pack.toml") {
            let mut config_string = String::new();
            config.read_to_string(&mut config_string).unwrap();

            PackageConfig::from_toml(&config_string) // TODO: result
        } else {
            panic!("Could not find Pack.toml") // TODO: error: Pack.toml not found
        };

        self.rooms.insert(
            room_code.clone(),
            (
                room_id,
                Room::new(config.into(), room_code.clone(), sender),
            ),
        );
        self.last_room_id.next();

        room_code
    }

    fn remove_room(&mut self, room_code: &RoomCode) {
        let (id, _) = self.rooms.remove(room_code).unwrap();

        std::fs::remove_file(format!("{}/{}_game.qua", UPLOADS_DIRECTORY, id)).unwrap();
    }

    fn get_package(&self, room_code: &RoomCode) -> Vec<u8> {
        let (id, _) = self.rooms.get(room_code).unwrap();

        let mut file = File::open(format!("{}/{}_game.qua", UPLOADS_DIRECTORY, id))
            .expect("Failed to open file");

        let mut file_data: Vec<u8> = vec![];
        file.read_to_end(&mut file_data)
            .expect("Failed to read file");

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

impl Default for RoomServiceInner {
    fn default() -> Self {
        Self {
            rooms: HashMap::new(),
            last_room_id: RoomId::min(),
        }
    }
}
