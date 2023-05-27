use dioxus::prelude::*;

pub fn game_info(cx: Scope) -> Element {
    cx.render(rsx! {
        div { class: "game-info",
            div { class: "pannel",
                div { class: "header", "INFO" }
                div { class: "text", "Some important information" }
            }
        }
    })
}

