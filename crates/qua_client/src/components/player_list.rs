use dioxus::prelude::*;

pub fn player_list(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "player-list",
        }
    })
}

