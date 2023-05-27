use std::{sync::Arc, time::Duration};

use dioxus::prelude::*;
use dioxus_router::*;
use qua_game::game::{ClientMessage, InputEvent};
use tokio::sync::Mutex;
use wasm_sockets::{Message, PollingClient};

use crate::Connection;

use super::prelude::GameTimer;

pub fn game_timer(cx: Scope) -> Element {
    let timer = use_shared_state::<GameTimer>(cx).unwrap().clone();

    let mut maybe_connection = use_shared_state::<Connection>(cx).unwrap().write_silent();

    let mut client: Arc<Mutex<PollingClient>> = if let Some(connection) = &*maybe_connection {
        connection.clone()
    } else {
        panic!("There is no socket connection!");
    };

    let future = use_future(cx, (), |_| async move {
        if let Some(time) = timer.read().0 {
            let mut interval = async_timer::Interval::platform_new(time);
            let wanna_send = serde_json::to_string(&ClientMessage::Input(InputEvent::Timeout))
                .expect("Failed to serialize");

            interval.wait().await;
            client
                .lock()
                .await
                .send_string(&wanna_send)
                .expect("Failed to send sync request");
        }
    });

    cx.render(rsx! { div {} })
}
