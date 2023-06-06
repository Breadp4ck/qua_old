use dioxus::prelude::*;
use qua_game::prelude::*;

use crate::Connection;

#[derive(Props)]
pub struct GameButtonProps<'game> {
    hidden: bool,
    disabled: bool,
    text: &'game str,
    color: &'game str,
    event: ClientMessage,
}

pub fn game_button<'game>(cx: Scope<'game, GameButtonProps<'game>>) -> Element {
    let maybe_connection = use_shared_state::<Connection>(cx).unwrap().write_silent();

    if cx.props.hidden {
        return cx.render(rsx! { div {} });
    }

    let client = if let Some(connection) = &*maybe_connection {
        Some(connection.clone())
    } else {
        None
    };

    let json = serde_json::to_string(&cx.props.event).unwrap();

    let press = move |_| {
        to_owned!(client, json);

        cx.spawn({
            async move {
                if let Some(client) = client {
                    let client = client.lock().await;
                    client.send_string(&json);
                }
            }
        });
    };

    if cx.props.disabled {
        cx.render(rsx! { div { class: "game-button-disabled {cx.props.color}-dark", "{cx.props.text}" } })
    } else {
        cx.render(rsx! { div { class: "game-button {cx.props.color}", onclick: press, "{cx.props.text}" } })
    }
}
