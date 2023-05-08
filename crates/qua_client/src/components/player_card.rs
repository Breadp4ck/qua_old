use dioxus::prelude::*;

#[derive(Props)]
pub struct PlayerCardProps<'player> {
    username: &'player str,
}

pub fn player_card<'player>(cx: Scope<'player, PlayerCardProps<'player>>) -> Element {
    cx.render(rsx! {
        div {
            class: "player-card",
            "{cx.props.username}"
        }
    })
}
