use dioxus::prelude::*;
use dioxus_router::*;
use fermi::use_read;

use crate::ROOM_CODE;

pub fn nav(cx: Scope) -> Element {
    let room_code = use_read(cx, ROOM_CODE);

    let room_code = if let Some(room_code) = room_code {
        Some(room_code.to_string())
    } else {
        None
    };

    cx.render(rsx! {
        nav {
            // Left side of the navbar
            Link {
                to: "/preferenes",
                class: "to-left",
                img {
                    class: "nav-btn-icon",
                    src: "assets/icons/settings-outline.svg",
                    alt: "P"
                }
            },
            Link {
                to: "/menu",
                class: "to-left",
                img {
                    class: "nav-btn-icon",
                    src: "assets/icons/home-outline.svg",
                    alt: "P"
                }
            },
            // Right side of the navbar
            Link {
                to: "/notifications",
                class: "to-right",
                img {
                    class: "nav-btn-icon",
                    src: "assets/icons/info-outline.svg",
                    alt: "P"
                }
            },
            if room_code.is_some() {
                rsx!(
                    Link {
                        to: "/game-info",
                        class: "to-right impressive",
                        "{room_code.unwrap().to_string()}"
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
            },
        }
    })
}
