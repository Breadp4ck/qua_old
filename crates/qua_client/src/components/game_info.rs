use dioxus::prelude::*;
use fermi::use_read;

use crate::INFO;

pub fn game_info(cx: Scope) -> Element {
    let info = use_read(cx, INFO);

    cx.render(rsx! {
        div { class: "game-info",
            div { class: "pannel",
                div { class: "header", "INFO" }
                div { class: "text", "{info}" }
            }
        }
    })
}

