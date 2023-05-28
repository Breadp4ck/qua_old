use std::{sync::Arc, time::Duration};

use dioxus::prelude::*;
use fermi::{use_read, use_set};
use qua_game::prelude::*;
use tokio::sync::Mutex;
use wasm_sockets::PollingClient;

use crate::{Connection, TIMER};

use super::prelude::GameTimer;

pub fn game_timer(cx: Scope) -> Element {
    let timer = use_shared_state::<Option<Duration>>(cx).unwrap();
    let maybe_connection = use_shared_state::<Connection>(cx).unwrap().write_silent();

    let client: Arc<Mutex<PollingClient>> = if let Some(connection) = &*maybe_connection {
        connection.clone()
    } else {
        panic!("There is no socket connection!");
    };

    let time = { timer.read().clone() };

    let loaded_timer = use_future(cx, &(time,), |(time,)| {
        to_owned!(timer);

        async move {
            if let Some(time) = time {
                let mut interval = async_timer::Interval::platform_new(time.clone());
                let wanna_send = serde_json::to_string(&ClientMessage::Input(InputEvent::Timeout))
                    .expect("Failed to serialize");

                interval.wait().await;

                let mut timer = timer.write_silent();
                *timer = None;

                client
                    .lock()
                    .await
                    .send_string(&wanna_send)
                    .expect("Failed to send sync request");
            }
        }
    });

    cx.render(rsx! { div {} })
}
