use axum::extract::ws::{Message, WebSocket};
use futures::{sink::SinkExt, stream::StreamExt};
use qua_game::{game::Game, package::prelude::Package, person::Person};

pub struct Room {
    game: Game,
}

impl Room {
    pub fn new(package: Package) -> Self {
        Self {
            game: Game::new(package),
        }
    }

    pub fn add_person(&mut self, socket: WebSocket, person: Person) {
        match person {
            Person::Player(player) => self.game.add_player(player),
            Person::Host(host) => self.game.add_host(host),
        }

        let (mut sender, mut receiver) = socket.split();

        tokio::spawn(async move {
            while let Some(message) = receiver.next().await {
                if let Ok(message) = message {
                    match message {
                        Message::Text(text) => {
                            let event = serde_json::from_str::<Event>(&text).unwrap();
                        }
                        Message::Binary(_) => {}
                        Message::Ping(_) => (),
                        Message::Pong(_) => (),
                        Message::Close(_) => (),
                    }
                }
            }
        });
    }
}
