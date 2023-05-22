use log::*;
use std::{collections::HashMap, sync::Arc};

use axum::extract::ws::{Message, WebSocket};
use futures::{
    stream::{SplitSink, StreamExt},
    SinkExt,
};
use qua_game::{
    game::{ClientMessage, Game, ServerMessage},
    package::prelude::Package,
    person::{Person, PersonName},
};
use tokio::sync::Mutex;

type Connections = HashMap<PersonName, SplitSink<WebSocket, Message>>;

pub struct Room {
    game: Arc<Mutex<Game>>,
    connections: Arc<Mutex<Connections>>,
}

impl Room {
    pub fn new(package: Package) -> Self {
        Self {
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

                                    let mut senders = senders.lock().await;
                                    Self::broadcast(
                                        &mut senders,
                                        &ServerMessage::Input(event, author.clone()),
                                    )
                                    .await;
                                }
                                ClientMessage::StatelessInput(event) => {
                                    let mut game = game.lock().await;
                                    game.handle_stateless_input(&event, &author);

                                    let mut senders = senders.lock().await;
                                    Self::broadcast(
                                        &mut senders,
                                        &ServerMessage::StatelessInput(event, author.clone()),
                                    )
                                    .await;
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
                            // TODO: remove sender from senders, close socket
                            // and send broadcast PlayerDisconnected event
                            info!("Person disconnected.");
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
