use dioxus::prelude::*;

pub fn game_host_card(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            "host_card"
        }
    })
}

