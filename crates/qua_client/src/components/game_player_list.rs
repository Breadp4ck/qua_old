use dioxus::prelude::*;
use qua_game::game::Game;
use qua_game::person::Personality;

use crate::components::prelude::*;

use super::prelude::UpdatePlayers;

pub fn game_player_list(cx: Scope) -> Element {
    let game = use_shared_state::<Game>(cx).unwrap();
    let players = use_shared_state::<UpdatePlayers>(cx).unwrap();

    let players = game.read().get_players();

    let players_rendered = players.iter().map(|player| {
        let player_name = player.name().to_string();
    });

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
