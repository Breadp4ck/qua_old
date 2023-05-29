use crate::*;
use dioxus::prelude::*;

pub fn join(cx: Scope) -> Element {
    cx.render(rsx! {
        div { class: "center-screen menu", join_room_form {} }
    })
}
