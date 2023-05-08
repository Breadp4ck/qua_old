use crate::components::prelude::*;
use dioxus::prelude::*;

pub fn game(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "center-screen game",
            div {
                class: "players",
                player_card { username: "player1" },
                player_card { username: "player2" },
                player_card { username: "player3" },
            }
            div {
                class: "tv",
                "tv"
            }
            div {
                class: "info",
                "info"
            }
        }
    })
}
