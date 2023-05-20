use std::sync::Arc;

use dioxus::prelude::*;
use fermi::{use_read, use_set, Atom, AtomRoot};
use log::*;
use qua_game::{
    game::{ClientMessage, Game, GameState, ServerMessage},
    package::prelude::Package,
};
use tokio::sync::Mutex;
use wasm_sockets::{Message, PollingClient};

use crate::{services::prelude::RoomService, TICKET, Connection};

use super::prelude::*;

#[derive(Clone)]
struct GameSharedState {
    board: UseSharedState<UpdateBoard>,
    players: UseSharedState<UpdatePlayers>,
    host: UseSharedState<UpdateHost>,
    info: UseSharedState<UpdateInfo>,
}

async fn ws(game: UseSharedState<Game>, state: GameSharedState, mut client: Arc<Mutex<PollingClient>>) {
    let mut interval = async_timer::Interval::platform_new(core::time::Duration::from_secs(1));

    let wanna_send = serde_json::to_string(&ClientMessage::SyncRequest).expect("Failed to serialize");

    interval.wait().await;
    client.lock().await.send_string(&wanna_send).expect("Failed to send sync request");
    info!("Event listening started.");

    loop {
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

                            state.board.notify_consumers();
                        },
                        ServerMessage::StatelessInput(event, author) => {
                            game.write_silent().handle_stateless_input(&event, &author);

                            state.board.notify_consumers();
                        },
                        ServerMessage::SyncResponse(synced_game) => {
                            game.write_silent().sync(synced_game);

                            state.board.notify_consumers();
                            state.players.notify_consumers();
                            state.host.notify_consumers();
                            info!("The game have been synced.");
                        }
                        ServerMessage::PersonConnected(person) => {
                            game.write_silent().add_person(person.clone());

                            state.players.notify_consumers();
                            info!("{} has joined the game.", person.name().to_string());
                        },
                        ServerMessage::PersonDisconnected(name) => {
                            game.write_silent().remove_person(name.clone());

                            state.players.notify_consumers();
                            info!("{} has disconnected.", name.to_string());
                        },
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
    let board = use_shared_state::<UpdateBoard>(cx).unwrap();
    let players = use_shared_state::<UpdatePlayers>(cx).unwrap();
    let host = use_shared_state::<UpdateHost>(cx).unwrap();
    let info = use_shared_state::<UpdateInfo>(cx).unwrap();

    let state = GameSharedState {
        board: board.clone(),
        players: players.clone(),
        host: host.clone(),
        info: info.clone(),
    };

    let mut client: Arc<Mutex<PollingClient>> = if let Some(connection) = &*maybe_connection {
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

    let _: &Coroutine<()> = use_coroutine(cx, |_| ws(game.clone(), state, client));

    cx.render(rsx! { div {} })
}
