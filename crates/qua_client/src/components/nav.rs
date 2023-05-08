use dioxus::prelude::*;
use dioxus_router::*;

pub fn nav(cx: Scope) -> Element {
    cx.render(rsx! {
        nav {
            Link { to: "/preferenes", class: "to-left", "P"}
            Link { to: "/menu", class: "to-left", "H"}
            Link { to: "/notifications", class: "to-right", "I"}
            Link { to: "/account", class: "to-right impressive", "Account INFO"}
        }
    })
}
