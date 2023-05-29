use std::time::Duration;

use dioxus::prelude::*;
use qua_game::prelude::*;

use crate::*;

pub fn game_count_wrong_button(cx: Scope) -> Element {
    let state = use_shared_state::<StateUpdate>(cx).unwrap();

    let (disabled, hidden) = if let StateUpdate::QuaAnswer = &*state.read() {
        (false, false)
    } else {
        (false, true)
    };

    cx.render(rsx! {
        div {
            game_button {
                hidden: hidden,
                disabled: disabled,
                text: "wrong",
                color: "accent-bg-red",
                event: ClientMessage::Input(InputEvent::CountWrong)
            }
        }
    })
}
