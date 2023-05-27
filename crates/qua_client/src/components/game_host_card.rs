use dioxus::prelude::*;

#[derive(Props)]
pub struct GameHostCardProps<'host> {
    username: &'host str,
}

pub fn game_host_card<'host>(cx: Scope<'host, GameHostCardProps<'host>>) -> Element {
    cx.render(rsx! {
        div { class: "host-card",
            div { class: "avatar" }
            div { class: "info", div { class: "username", "{cx.props.username}" } }
        }
    })
}

