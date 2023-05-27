use dioxus::prelude::*;
use qua_game::prelude::*;

use crate::*;

pub fn game_player_list(cx: Scope) -> Element {
    let game = use_shared_state::<Game>(cx).unwrap();
    let _ = use_shared_state::<PlayerUpdate>(cx).unwrap();

    let players = game.read().get_players();

    cx.render(rsx! {
        div { class: "player-list",
            for player in players.iter() {
                game_player_card {
                    username: "{&player.name().clone().to_string()}",
                    scores: player.scores(),
                    lead: false
                }
            }
        }
    })
}
