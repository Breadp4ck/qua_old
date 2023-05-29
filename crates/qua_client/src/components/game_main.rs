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
    use_shared_state_provider(cx, || StateUpdate::Init);
    use_shared_state_provider(cx, || None::<Duration>);
    let person_type = use_read(cx, PERSON_TYPE);

    cx.render(rsx! {
        game_handler {}
        game_timer {}
        div { class: "center-screen",
            div { class: "game",
                div { class: "left", game_player_list {} }
                div { class: "center",
                    div { class: "game-board",
                        game_board {}
                        game_progress_bar {}
                    }
                }
                div { class: "right",
                    div { class: "side",
                        div { class: "top",
                            game_host {}
                            game_info {}
                        }
                        div { class: "bottom",
                            match person_type {
                                PersonType::Lead | PersonType::Player => rsx!{game_answer_button {}},
                                PersonType::Host => rsx!{
                                    game_begin_button {}
                                    game_timeout_button {}
                                    game_count_correct_button {}
                                    game_count_wrong_button {}
                                },
                            }
                        }
                    }
                }
            }
        }
    })
}
