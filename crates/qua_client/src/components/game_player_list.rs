use dioxus::prelude::*;
use qua_game::game::Game;
use qua_game::person::Personality;
use qua_game::scores::Scores;

use crate::components::prelude::*;

use super::prelude::UpdatePlayers;

pub fn game_player_list(cx: Scope) -> Element {
    let game = use_shared_state::<Game>(cx).unwrap();
    let players = use_shared_state::<UpdatePlayers>(cx).unwrap();

    let players = game.read().get_players();

    let players_rendered = players.iter().map(|player| {
        let player_name = player.name().to_string();
        log::info!("Player: {}", player_name);
    });

    log::info!("Player list is rendered with {} players", players.len());
    log::info!("Game: {}", serde_json::to_string(&*game.clone().read()).unwrap());

    cx.render(rsx! {
        div {
            class: "player-list",
            game_player_card {
                username: "Тестовый игрок 1",
                scores: Scores::from(250),
                lead: false
            },
            game_player_card {
                username: "Тестовый игрок 2",
                scores: Scores::from(32000),
                lead: true
            }
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
