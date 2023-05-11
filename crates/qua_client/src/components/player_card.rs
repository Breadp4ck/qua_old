use dioxus::prelude::*;
use qua_game::scores::Scores;

#[derive(Props)]
pub struct PlayerCardProps<'player> {
    username: &'player str,
    scores: Scores,
    lead: bool,
}

pub fn player_card<'player>(cx: Scope<'player, PlayerCardProps<'player>>) -> Element {
    cx.render(rsx! {
        div {
            class: "player-card",
            if cx.props.lead {
                "{cx.props.username} {cx.props.scores} lead"
            } else {
                "{cx.props.username} {cx.props.scores}"
            }
        }
    })
}
