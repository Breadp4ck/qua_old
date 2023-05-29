use dioxus::prelude::*;
use fermi::use_read;
use qua_game::prelude::*;

use crate::*;

#[derive(PartialEq, Props)]
pub struct GamePlayerCardProps {
    username: PersonName,
    scores: Scores,
    lead: bool,
}

pub fn game_player_card(cx: Scope<GamePlayerCardProps>) -> Element {
    let maybe_connection = use_shared_state::<Connection>(cx).unwrap().write_silent();
    let person_type = use_read(cx, PERSON_TYPE).to_owned();

    let client = if let Some(connection) = &*maybe_connection {
        Some(connection.clone())
    } else {
        None
    };

    let username = cx.props.username.clone();

    let change_lead = move |_| {
        to_owned!(client, username);

        cx.spawn({
            async move {
                let json = serde_json::to_string(
                    &ClientMessage::StatelessInput(StatelessInputEvent::AssignLeadPlayer(username)),
                ).expect("Can not parse json! May be qua_game is different?");

                if let Some(client) = client {
                    let client = client.lock().await;
                    client.send_string(&json);
                }
            }
        });
    };

    let client = if let Some(connection) = &*maybe_connection {
        Some(connection.clone())
    } else {
        None
    };

    cx.render(rsx! {
        div { class: "player-card",
            div { class: "avatar" }
            div { class: "info",
                div { class: "username", "{cx.props.username}" }
                div { class: "scores", "{cx.props.scores}" }
            }
            if cx.props.lead {
                rsx! { div { class: "lead" } }
            } else if let PersonType::Host = person_type {
                rsx! { div { onclick: change_lead, class: "lead-placeholder" } }
            }
        }
    })
}
