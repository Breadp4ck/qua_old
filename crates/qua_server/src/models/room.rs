use std::sync::Arc;

use axum::extract::ws::{Message, WebSocket};
use futures::{stream::{SplitSink, StreamExt}, SinkExt};
use qua_game::{
    game::{Game, InputEvent},
    package::prelude::Package,
    person::{Person, PersonName},
};
use tokio::sync::Mutex;

pub struct Room {
    game: Arc<Mutex<Game>>,
    senders: Arc<Mutex<Vec<SplitSink<WebSocket, Message>>>>,
}

impl Room {
    pub fn new(package: Package) -> Self {
        Self {
            game: Arc::new(Mutex::new(Game::new(package))),
            senders: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub async fn add_person(&mut self, socket: WebSocket, person: Person) {
        let mut author = person.clone();

        match person {
            Person::Player(player) => self.game.lock().await.add_player(player),
            Person::Host(host) => self.game.lock().await.add_host(host),
        }

        let (sender, mut receiver) = socket.split();
        let game = self.game.clone();

        self.senders.lock().await.push(sender);
        let senders = self.senders.clone();

        tokio::spawn(async move {
            while let Some(message) = receiver.next().await {
                if let Ok(message) = message {
                    match message {
                        Message::Text(text) => {
                            let (event, _) = serde_json::from_str::<(InputEvent, PersonName)>(&text).unwrap();
                            // game.lock().await.handle_event(&event, &mut author);

                            let mut senders = senders.lock().await;
                            Self::broadcast(&mut senders, &event, &author.name()).await;

                            println!("Event!");
                        }
                        Message::Binary(_) => {}
                        Message::Ping(_) => {}
                        Message::Pong(_) => {}
                        Message::Close(_) => {
                            println!("Connection closed :(");
                        }
                    }
                }
            }
        });
    }

    async fn broadcast(senders: &mut Vec<SplitSink<WebSocket, Message>>, event: &InputEvent, author_name: &PersonName) {
        for sender in senders.iter_mut() {
            if sender.send(Message::Text(serde_json::to_string(&(event, author_name)).unwrap())).await.is_err() {
                eprintln!("Client disconnected");
            };
        }
    }
}
