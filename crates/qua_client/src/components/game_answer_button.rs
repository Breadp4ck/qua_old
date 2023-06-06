use std::time::Duration;

use dioxus::prelude::*;
use qua_game::prelude::*;

use crate::*;

pub fn game_answer_button(cx: Scope) -> Element {
    let maybe_connection = use_shared_state::<Connection>(cx).unwrap().write_silent();
    let state = use_shared_state::<StateUpdate>(cx).unwrap();
    let qua_time = use_shared_state::<QuaWaitingTime>(cx).unwrap();

    let (disabled, hidden) = match &*state.read() {
        StateUpdate::Init => (true, false),
        StateUpdate::Greet => (true, false),
        StateUpdate::Overview => (true, false),
        StateUpdate::Picking => (true, false),
        StateUpdate::QuestionAppearance => (true, false),
        StateUpdate::QuestionMatter => (true, false),
        StateUpdate::QuestionAsking => (true, false),
        StateUpdate::QuaWaiting => (false, false),
        StateUpdate::QuaQueue => (false, false),
        StateUpdate::QuaAnswer => (true, false),
        StateUpdate::QuestionAnswer => (true, false),
        StateUpdate::Ending => (false, true),
    };

    if hidden {
        return cx.render(rsx! { div {} });
    }

    let client = if let Some(connection) = &*maybe_connection {
        Some(connection.clone())
    } else {
        None
    };

    let press = move |_| {
        to_owned!(client, qua_time);

        cx.spawn({
            async move {
                if let Some(client) = client {
                    let current_time = TimeService::now();

                    if let Some(time) = qua_time.read().0 {
                        let json = serde_json::to_string(&ClientMessage::Input(
                            InputEvent::Answer(current_time.duration_since(time).unwrap()),
                        ))
                        .unwrap();

                        let client = client.lock().await;
                        client.send_string(&json);
                    }

                    *qua_time.write() = QuaWaitingTime(None);
                }
            }
        });
    };

    if disabled {
        cx.render(rsx! { div { class: "game-button-disabled accent-bg-yellow-dark", "qua!" } })
    } else {
        cx.render(rsx! { div { class: "game-button accent-bg-yellow", onclick: press, "qua!" } })
    }
}
