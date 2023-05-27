use dioxus::prelude::*;
use qua_game::{
    game::Game,
    person::Personality,
};
use super::prelude::*;

pub fn game_host(cx: Scope) -> Element {
    let game = use_shared_state::<Game>(cx).unwrap();
    let host = use_shared_state::<UpdateHost>(cx).unwrap();

    match game.read().get_host() {
        Some(host) => log::info!("{:?}", host.name()),
        None => log::info!("THERE IS NO HOST"),
    }

    if let Some(host) = game.read().get_host() {
        cx.render(rsx! {
            div { class: "game-host", game_host_card { username: "{host.name()}" } }
        })
    } else {
        cx.render(rsx! {
            div { class: "game-host", game_host_card { username: "Без хоста :(" } }
        })
    }
}
