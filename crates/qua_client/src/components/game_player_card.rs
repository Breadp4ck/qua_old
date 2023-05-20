use dioxus::prelude::*;
use qua_game::scores::Scores;

#[derive(Props)]
pub struct GamePlayerCardProps<'player> {
    username: &'player str,
    scores: Scores,
    lead: bool,
}

pub fn game_player_card<'player>(cx: Scope<'player, GamePlayerCardProps<'player>>) -> Element {
    cx.render(rsx! {
        div {
            class: "player-card",
            "{cx.props.username}"
        }
    })
}
