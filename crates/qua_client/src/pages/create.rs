use crate::*;
use dioxus::prelude::*;

pub fn create(cx: Scope) -> Element {
    cx.render(rsx! {
        div { class: "center-screen menu", create_room_form {} }
    })
}
