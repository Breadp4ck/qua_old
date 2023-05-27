use std::{sync::Arc, time::Duration};

use crate::InnerConnection;

use super::prelude::*;
use dioxus::prelude::*;
use qua_game::{game::{Game, Question, Round}, package::prelude::PackageState};
use tokio::sync::Mutex;
use wasm_sockets::PollingClient;

pub enum UpdateBoard {
    Message(String),
    Question(Question),
    Board(Round),
}
pub struct UpdatePlayers {}
pub struct UpdateHost {}
pub struct UpdateInfo;
pub struct GameConnectionEstablished(pub bool, pub Option<Arc<Mutex<PollingClient>>>);
pub struct GameTimer(pub Option<Duration>); // autostart timer

pub fn game_main(cx: Scope) -> Element {
    use_shared_state_provider(cx, || Game::new(PackageState::default()));
    use_shared_state_provider(cx, || None::<InnerConnection>);
    use_shared_state_provider(cx, || UpdateBoard::Board(Round::Normal(0.into())));
    use_shared_state_provider(cx, || UpdatePlayers {});
    use_shared_state_provider(cx, || UpdateHost {});
    use_shared_state_provider(cx, || UpdateInfo {});
    use_shared_state_provider(cx, || GameTimer(None));

    cx.render(rsx! {
        game_handler {}
        game_timer {}
        div { class: "center-screen",
            div { class: "game",
                div { class: "left", game_player_list {} }
                div { class: "center", game_board {} }
                div { class: "right",
                    div { class: "side",
                        div { class: "top",
                            game_host {}
                            game_info {}
                        }
                        div { class: "bottom",
                            game_answer_button {}
                            game_timeout_button {}
                        }
                    }
                }
            }
        }
    })
}
