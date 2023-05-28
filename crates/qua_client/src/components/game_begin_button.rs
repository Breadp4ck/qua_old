use std::time::Duration;

use dioxus::prelude::*;
use qua_game::prelude::*;

use crate::*;

pub fn game_begin_button(cx: Scope) -> Element {
    let state = use_shared_state::<StateUpdate>(cx).unwrap();

    let (disabled, hidden) = if let StateUpdate::Init = &*state.read() {
        (false, false)
    } else {
        (false, true)
    };

    cx.render(rsx! {
        div { 
            game_button {
                hidden: hidden,
                disabled: disabled,
                text: "begin!",
                color: "accent-bg-yellow",
                event: ClientMessage::Input(InputEvent::Begin),
            }
        }
    })
}
