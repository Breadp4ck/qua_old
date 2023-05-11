use crate::components::prelude::*;
use dioxus::prelude::*;

pub fn home(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "center-screen menu",
            menu_card { to: "/create", title: "Create", icon: "1" },
            menu_card { to: "/join", title: "Join", icon: "2" },
            menu_card { to: "/package", title: "Create Package", icon: "3" },
        }
    })
}
