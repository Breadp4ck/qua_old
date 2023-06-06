use std::{rc::Rc, sync::Arc, time::Duration};

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
    state: UseSharedState<StateUpdate>,
}

async fn ws(
    game: UseSharedState<Game>,
    state: GameSharedState,
    client: Arc<Mutex<PollingClient>>,
    synced: bool,
    timer: UseSharedState<GameTimer>,
    qua_time: UseSharedState<QuaWaitingTime>,
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
                            GameEvent::State(update) => {
                                let mut time = timer.write();
                                *time = match update {
                                    StateUpdate::Init => GameTimer(None),
                                    StateUpdate::Greet => GameTimer(Some(Duration::from_secs(2))),
                                    StateUpdate::Overview => GameTimer(Some(Duration::from_secs(2))),
                                    StateUpdate::Picking => GameTimer(None),
                                    StateUpdate::QuestionAppearance => GameTimer(Some(Duration::from_secs(1))),
                                    StateUpdate::QuestionMatter => GameTimer(Some(Duration::from_secs(2))),
                                    StateUpdate::QuestionAsking => GameTimer(Some(Duration::from_secs(1))),
                                    StateUpdate::QuaWaiting => GameTimer(Some(Duration::from_secs(10))),
                                    StateUpdate::QuaQueue => GameTimer(Some(Duration::from_secs(1))),
                                    StateUpdate::QuaAnswer => GameTimer(Some(Duration::from_secs(10))),
                                    StateUpdate::QuestionAnswer => GameTimer(Some(Duration::from_secs(2))),
                                    StateUpdate::Ending => GameTimer(None),
                                };

                                let mut qua_time = qua_time.write_silent();
                                if let StateUpdate::QuaWaiting = update {
                                    *qua_time = QuaWaitingTime(Some(TimeService::now()));
                                }

                                let mut state = state.state.write();
                                *state = update;
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
    let timer = use_shared_state::<GameTimer>(cx).unwrap();
    let qua_time = use_shared_state::<QuaWaitingTime>(cx).unwrap();

    let mut maybe_connection = use_shared_state::<Connection>(cx).unwrap().write_silent();
    let board = use_shared_state::<BoardUpdate>(cx).unwrap();
    let players = use_shared_state::<PlayerUpdate>(cx).unwrap();
    let host = use_shared_state::<HostUpdate>(cx).unwrap();
    let state = use_shared_state::<StateUpdate>(cx).unwrap();

    let state = GameSharedState {
        board: board.clone(),
        players: players.clone(),
        host: host.clone(),
        state: state.clone(),
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

    let _: &Coroutine<()> = use_coroutine(cx, |_| {
        to_owned!(timer, qua_time);
        ws(game.clone(), state, client, synced, timer, qua_time)
    });

    cx.render(rsx! { div {} })
}
