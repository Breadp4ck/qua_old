use dioxus::prelude::*;
use dioxus_router::*;

#[derive(Props)]
pub struct MenuCardProps<'menu> {
    to: &'menu str,
    header: &'menu str,
    description: &'menu str,
    color_accent_class: &'menu str,
    icon_src: &'menu str,
    icon_alt: &'menu str,
}

pub fn menu_card<'menu>(cx: Scope<'menu, MenuCardProps<'menu>>) -> Element {
    cx.render(rsx! {
        Link {
            to: "{cx.props.to}",
            class: "menu-card",
            div {
                class: "menu-card-top {cx.props.color_accent_class}",
                span {
                    img {
                        class: "menu-card-icon",
                        src: "{cx.props.icon_src}",
                        alt: "{cx.props.icon_alt}"
                    }
                },
            },
            div {
                class: "menu-card-bottom",
                div {
                    class: "header",
                    "{cx.props.header}"
                }
                div {
                    class: "paragraph",
                    "{cx.props.description}"
                }
            },
        }
    })
}
