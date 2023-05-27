use log::*;
use room_code::RoomCode;
use std::{collections::HashMap, sync::Arc};

use axum::extract::ws::{Message, WebSocket};
use futures::{
    stream::{SplitSink, StreamExt},
    SinkExt,
};
use qua_game::{
    game::{ClientMessage, Game, ServerMessage},
    package::prelude::PackageState,
    person::{Person, PersonName},
};
use tokio::sync::{mpsc::UnboundedSender, Mutex};

use crate::{models::room_code, services::prelude::*};

type Connections = HashMap<PersonName, SplitSink<WebSocket, Message>>;

pub struct Room {
    code: RoomCode,
    room_service_sender: UnboundedSender<RoomServiceEvent>,
    game: Arc<Mutex<Game>>,
    connections: Arc<Mutex<Connections>>,
}

impl Room {
    pub fn new(
        package: PackageState,
        code: RoomCode,
        room_service_sender: UnboundedSender<RoomServiceEvent>,
    ) -> Self {
        Self {
            code,
            room_service_sender,
            game: Arc::new(Mutex::new(Game::new(package))),
            connections: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn add_person(&mut self, socket: WebSocket, person: Person) {
        let author = person.name().clone();
        self.game.lock().await.add_person(person.clone());

        let (sender, mut receiver) = socket.split();
        let game = self.game.clone();

        self.connections.lock().await.insert(author.clone(), sender);
        let senders = self.connections.clone();
        let room_code = self.code.clone();
        let room_service_sender = self.room_service_sender.clone();

        tokio::spawn(async move {
            info!("Person connected.");

            {
                let mut senders = senders.lock().await;
                Self::broadcast(&mut senders, &ServerMessage::PersonConnected(person)).await;
            }

            while let Some(message) = receiver.next().await {
                if let Ok(message) = message {
                    match message {
                        Message::Text(text) => {
                            let Ok(message) = serde_json::from_str::<ClientMessage>(&text) else {
                                continue;
                            };

                            info!("Got message from client: {}", text);

                            match message {
                                ClientMessage::Input(event) => {
                                    let mut game = game.lock().await;
                                    game.handle_input(&event, &author);

                                    if game.abandon_events() {
                                        let mut senders = senders.lock().await;
                                        Self::broadcast(
                                            &mut senders,
                                            &ServerMessage::Input(event, author.clone()),
                                        )
                                        .await;
                                    }
                                }
                                ClientMessage::StatelessInput(event) => {
                                    let mut game = game.lock().await;
                                    game.handle_stateless_input(&event, &author);

                                    if game.abandon_events() {
                                        let mut senders = senders.lock().await;
                                        Self::broadcast(
                                            &mut senders,
                                            &ServerMessage::StatelessInput(event, author.clone()),
                                        )
                                        .await;
                                    }
                                }
                                ClientMessage::SyncRequest => {
                                    let game = game.lock().await;

                                    let mut senders = senders.lock().await;
                                    Self::send(
                                        &mut senders,
                                        &author,
                                        &ServerMessage::SyncResponse(game.clone()),
                                    )
                                    .await;
                                }
                            }
                        }
                        Message::Close(_) => {
                            info!("Person disconnected.");
                            let mut senders = senders.lock().await;
                            senders.remove(&author);

                            Self::broadcast(&mut senders, &ServerMessage::PersonDisconnected(author)).await;

                            if senders.len() == 0 {
                                match room_service_sender
                                    .send(RoomServiceEvent::RoomEmptied(room_code.clone()))
                                {
                                    Ok(_) => {
                                        info!("Room with code '{}' deleted.", room_code.to_string())
                                    }
                                    Err(e) => warn!(
                                        "Room with code '{}' is not properly deleted: {}.",
                                        room_code.to_string(),
                                        e
                                    ),
                                }
                            }

                            return;
                        }
                        _ => (),
                    }
                }
            }
        });
    }

    async fn send(
        senders: &mut Connections,
        receiver: &PersonName,
        server_message: &ServerMessage,
    ) {
        let sender = senders.get_mut(receiver).unwrap();
        let message = Message::Text(serde_json::to_string(&server_message).unwrap());
        if sender.send(message).await.is_err() {
            eprintln!("Client disconnected");
        };
    }

    async fn broadcast(senders: &mut Connections, server_message: &ServerMessage) {
        for sender in senders.values_mut() {
            let message = Message::Text(serde_json::to_string(&server_message).unwrap());
            if sender.send(message).await.is_err() {
                eprintln!("Client disconnected");
            };
        }
    }
}
