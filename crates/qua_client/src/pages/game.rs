use crate::components::prelude::*;
use dioxus::prelude::*;

pub fn game(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "center-screen game",
            div {
                player_list {}
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
