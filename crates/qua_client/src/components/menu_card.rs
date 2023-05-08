use dioxus::prelude::*;
use dioxus_router::*;

#[derive(Props)]
pub struct MenuCardProps<'menu> {
    to: &'menu str,
    title: &'menu str,
    icon: &'menu str,
}

pub fn menu_card<'menu>(cx: Scope<'menu, MenuCardProps<'menu>>) -> Element {
    cx.render(rsx! {
        Link {
            to: "{cx.props.to}",
            class: "menu-card",
            div {
                class: "menu-card-top",
                span {
                    class: "material-symbols-outlined",
                    "{cx.props.icon}"
                },
            },
            div {
                class: "menu-card-bottom",
                "{cx.props.title}"
            },
        }
    })
}
