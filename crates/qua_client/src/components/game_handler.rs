use std::{sync::Arc, time::Duration};

use async_timer::Interval;
use dioxus::prelude::*;
use fermi::use_read;
use log::*;
use qua_game::prelude::*;
use tokio::sync::Mutex;
use wasm_sockets::{Message, PollingClient};

use crate::*;

#[derive(Clone)]
struct GameSharedState {
    board: UseSharedState<BoardUpdate>,
    players: UseSharedState<PlayerUpdate>,
    host: UseSharedState<HostUpdate>,
}

async fn ws(
    game: UseSharedState<Game>,
    state: GameSharedState,
    client: Arc<Mutex<PollingClient>>,
    synced: bool,
) {
    let mut interval = Interval::platform_new(Duration::from_millis(200));
    if !synced {
        let wanna_send =
            serde_json::to_string(&ClientMessage::SyncRequest).expect("Failed to serialize");

        // TODO: wait for client starts, timer is cringe
        interval.wait().await;
        client
            .lock()
            .await
            .send_string(&wanna_send)
            .expect("Failed to send sync request");
    }

    loop {
        // TODO: wait for client starts, timer is cringe
        interval.wait().await;

        for message in client.lock().await.receive() {
            log::info!("Got message: {:?}", message);
            match message {
                Message::Text(text) => {
                    let Ok(message) = serde_json::from_str::<ServerMessage>(&text) else {
                        continue;
                    };

                    match message {
                        ServerMessage::Input(event, author) => {
                            game.write_silent().handle_input(&event, &author);
                        }
                        ServerMessage::StatelessInput(event, author) => {
                            game.write_silent().handle_stateless_input(&event, &author);
                        }
                        ServerMessage::SyncResponse(synced_game) => {
                            game.write_silent().sync(synced_game);
                            info!("The game have been synced.");
                        }
                        ServerMessage::PersonConnected(person) => {
                            game.write_silent().add_person(person.clone());
                            info!("{} has joined the game.", person.name().to_string());
                        }
                        ServerMessage::PersonDisconnected(name) => {
                            game.write_silent().remove_person(name.clone());
                            info!("{} has disconnected.", name.to_string());
                        }
                    }

                    while let Some(event) = game.write_silent().event_try_recv() {
                        match event {
                            GameEvent::Board(update) => {
                                let mut board = state.board.write();
                                *board = update;
                            }
                            GameEvent::Player(update) => {
                                let mut players = state.players.write();
                                *players = update;
                            }
                            GameEvent::Host(update) => {
                                let mut host = state.host.write();
                                *host = update;
                            }
                        }
                    }
                }
                Message::Binary(_) => {}
            }
        }
    }
}

pub fn game_handler(cx: Scope) -> Element {
    let ticket = use_read(cx, TICKET);
    let game = use_shared_state::<Game>(cx).unwrap();

    let mut maybe_connection = use_shared_state::<Connection>(cx).unwrap().write_silent();
    let board = use_shared_state::<BoardUpdate>(cx).unwrap();
    let players = use_shared_state::<PlayerUpdate>(cx).unwrap();
    let host = use_shared_state::<HostUpdate>(cx).unwrap();

    let state = GameSharedState {
        board: board.clone(),
        players: players.clone(),
        host: host.clone(),
    };

    let mut synced = false;

    let client: Arc<Mutex<PollingClient>> = if let Some(connection) = &*maybe_connection {
        synced = true;
        connection.clone()
    } else {
        if let Some(ticket) = ticket {
            let connection = Arc::new(Mutex::new(RoomService::join_room(ticket)));
            *maybe_connection = Some(connection.clone());
            connection
        } else {
            panic!("AAAAA There is not ticket");
        }
    };

    let _: &Coroutine<()> = use_coroutine(cx, |_| ws(game.clone(), state, client, synced));

    cx.render(rsx! { div {} })
}
