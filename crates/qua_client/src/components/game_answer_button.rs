use std::time::Duration;

use dioxus::prelude::*;
use qua_game::prelude::*;

use crate::Connection;

pub fn game_answer_button(cx: Scope) -> Element {
    let maybe_connection = use_shared_state::<Connection>(cx)
        .unwrap()
        .write_silent();

    let client = if let Some(connection) = &*maybe_connection {
        Some(connection.clone())
    } else {
        None
    };

    let press = move |_| {
        to_owned!(client);

        cx.spawn({
            async move {
                if let Some(client) = client {
                    let client = client.lock().await;
                    client.send_string(
                        &serde_json::to_string(&ClientMessage::Input(
                            qua_game::game::InputEvent::Answer(Duration::new(1, 0)),
                        ))
                        .unwrap(),
                    );
                }
            }
        });
    };

    cx.render(rsx! {
        div { class: "game-button", onclick: press, div { class: "text", "qua!" } }
    })
}
