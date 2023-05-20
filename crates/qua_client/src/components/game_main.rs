use std::sync::Arc;

use crate::{Connection, InnerConnection};

use super::prelude::*;
use dioxus::prelude::*;
use qua_game::{game::Game, package::prelude::Package};
use tokio::sync::Mutex;
use wasm_sockets::PollingClient;

pub struct UpdateBoard;
pub struct UpdatePlayers;
pub struct UpdateHost;
pub struct UpdateInfo;
pub struct GameConnectionEstablished(pub bool, pub Option<Arc<Mutex<PollingClient>>>);

pub fn game_main(cx: Scope) -> Element {
    use_shared_state_provider(cx, || Game::new(Package::default()));
    use_shared_state_provider(cx, || None::<InnerConnection>);
    use_shared_state_provider(cx, || UpdateBoard {});
    use_shared_state_provider(cx, || UpdatePlayers {});
    use_shared_state_provider(cx, || UpdateHost {});
    use_shared_state_provider(cx, || UpdateInfo {});

    cx.render(rsx! {
        game_handler {},
        div {
            class: "center-screen game",
            div {
                class: "left",
                game_player_list {}
            }
            div {
                class: "center",
                game_board {}
            }
            div {
                class: "right",
                game_host_card {}
                game_info {}
                game_answer_button {}
            }
        }
    })
}
