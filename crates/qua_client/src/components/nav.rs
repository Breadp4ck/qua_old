use dioxus::prelude::*;
use dioxus_router::*;
use fermi::use_read;
use web_sys::window;

use crate::ROOM_CODE;

pub fn nav(cx: Scope) -> Element {
    let room_code = use_read(cx, ROOM_CODE);

    let copy_code = move |_| {
        to_owned!(room_code);

        cx.spawn({
            async move {
                if let Some(room_code) = room_code {
                    let clipboard = window().unwrap().navigator().clipboard().unwrap();
                    clipboard.write_text(&room_code.to_string());
                }
            }
        });
    };


    cx.render(rsx! {
        nav {
            // Left side of the navbar
            // Link { to: "/preferenes", class: "to-left", img { class: "nav-btn-icon", src: "assets/icons/settings-outline.svg", alt: "P" } }
            Link { to: "/menu", class: "to-left", img { class: "nav-btn-icon", src: "assets/icons/home-outline.svg", alt: "P" } }
            // Right side of the navbar
            // Link { to: "/notifications", class: "to-right", img { class: "nav-btn-icon", src: "assets/icons/info-outline.svg", alt: "P" } }

            if let Some(room_code) = room_code {
                rsx!(
                    div {
                        onclick: copy_code,
                        class: "to-right impressive",
                        "{room_code.to_string()}"
                    },
                    Link {
                        to: "/room",
                        class: "to-right impressive",
                        img {
                            class: "nav-btn-icon",
                            src: "assets/icons/arrow-ios-back-outline.svg",
                            alt: "P"
                        }
                    }
                )
            }
        }
    })
}
