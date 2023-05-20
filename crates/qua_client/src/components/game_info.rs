use dioxus::prelude::*;

pub fn game_info(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "game-info",
            "game_info"
        }
    })
}

