use dioxus::prelude::*;
use qua_game::prelude::*;

use crate::*;

pub fn game_player_list(cx: Scope) -> Element {
    let game = use_shared_state::<Game>(cx).unwrap();
    let _ = use_shared_state::<PlayerUpdate>(cx).unwrap();

    let players = game.read().get_players();
    let leader_name = game.read().get_leader_name();

    if let Some(leader_name) = leader_name {
        cx.render(rsx! {
            div { class: "player-list",
                for player in players.iter() {
                    game_player_card {
                        username: player.name().clone(),
                        scores: player.scores(),
                        lead: if leader_name == player.name().clone() { true } else { false }
                    }
                }
            }
        })
    } else {
        cx.render(rsx! {
            div { class: "player-list",
                for player in players.iter() {
                    game_player_card { username: player.name().clone(), scores: player.scores(), lead: false }
                }
            }
        })
    }
}
