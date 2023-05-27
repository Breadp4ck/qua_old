use std::time::Duration;

use dioxus::prelude::*;
use qua_game::game::{ClientMessage, Game};

use crate::Connection;

pub fn game_timeout_button(cx: Scope) -> Element {
    let game = use_shared_state::<Game>(cx).unwrap();
    let mut maybe_connection = use_shared_state::<Connection>(cx)
        .unwrap()
        .write_silent();

    let client = if let Some(connection) = &*maybe_connection {
        Some(connection.clone())
    } else {
        None
    };

    let press = move |_| {
        to_owned!(client);
        log::info!("Send timeout!");

        cx.spawn({
            async move {
                if let Some(client) = client {
                    let mut client = client.lock().await;
                    client.send_string(
                        &serde_json::to_string(&ClientMessage::Input(
                            qua_game::game::InputEvent::Timeout,
                        ))
                        .unwrap(),
                    );
                }
            }
        });
    };

    cx.render(rsx! {
        div { class: "game-button", onclick: press, div { class: "text", "next" } }
    })
}

