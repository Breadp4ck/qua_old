use crate::components::prelude::*;
use dioxus::prelude::*;

pub fn home(cx: Scope) -> Element {
    cx.render(rsx! {
        div { class: "center-screen menu",
            menu_card {
                to: "/create",
                header: "CREAETE",
                description: "Create new room",
                color_accent_class: "accent-bg-1",
                icon_src: "assets/icons/play-circle-outline.svg",
                icon_alt: "1"
            }
            menu_card {
                to: "/join",
                header: "JOIN",
                description: "Join to room",
                color_accent_class: "accent-bg-2",
                icon_src: "assets/icons/person-add-outline.svg",
                icon_alt: "2"
            }
            menu_card {
                to: "/package",
                header: "PACKAGE",
                description: "Create new package",
                color_accent_class: "accent-bg-3",
                icon_src: "assets/icons/briefcase-outline.svg",
                icon_alt: "3"
            }
        }
    })
}
