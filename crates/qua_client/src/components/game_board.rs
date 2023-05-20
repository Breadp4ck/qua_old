use dioxus::prelude::*;
use qua_game::game::Game;

use super::prelude::UpdateBoard;

pub fn game_board(cx: Scope) -> Element {
    let game = use_shared_state::<Game>(cx).unwrap();
    let board = use_shared_state::<UpdateBoard>(cx).unwrap();

    cx.render(rsx! {
        div {
            class: "game-board",
            "board"
        }
    })
}
