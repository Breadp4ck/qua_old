use dioxus::prelude::*;
use qua_game::game::{ClientMessage, Game};

use crate::Connection;

use super::prelude::GameConnectionEstablished;

pub fn game_answer_button(cx: Scope) -> Element {
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

        cx.spawn({
            async move {
                if let Some(client) = client {
                    let mut client = client.lock().await;
                    client.send_string(
                        &serde_json::to_string(&ClientMessage::Input(
                            qua_game::game::InputEvent::Answer,
                        ))
                        .unwrap(),
                    );
                }
            }
        });
    };

    cx.render(rsx! {
        div {
            onclick: press,
            "button"
        }
    })
}
