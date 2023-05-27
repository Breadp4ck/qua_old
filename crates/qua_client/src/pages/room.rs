use crate::components::prelude::*;
use dioxus::prelude::*;

pub fn room(cx: Scope) -> Element {
    cx.render(rsx! { game_main {} })
}
