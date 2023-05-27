use std::{sync::Arc, time::Duration};

use qua_game::prelude::*;

use crate::*;
use dioxus::prelude::*;
use tokio::sync::Mutex;
use wasm_sockets::PollingClient;

pub struct GameTimer(pub Option<Duration>); // autostart timer

pub fn game_main(cx: Scope) -> Element {
    use_shared_state_provider(cx, || Game::new(PackageState::default()));
    use_shared_state_provider(cx, || None::<InnerConnection>);
    use_shared_state_provider(cx, || BoardUpdate::Init);
    use_shared_state_provider(cx, || PlayerUpdate::Sync);
    use_shared_state_provider(cx, || HostUpdate::Sync);
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
