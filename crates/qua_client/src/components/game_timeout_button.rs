use dioxus::prelude::*;
use qua_game::prelude::*;

use crate::*;

pub fn game_timeout_button(cx: Scope) -> Element {
    let state = use_shared_state::<StateUpdate>(cx).unwrap();

    let (disabled, hidden) = match &*state.read() {
        StateUpdate::Init => (false, true),
        StateUpdate::Greet => (false, false),
        StateUpdate::Overview => (false, false),
        StateUpdate::Picking => (true, false),
        StateUpdate::QuestionAppearance => (false, false),
        StateUpdate::QuestionMatter => (false, false),
        StateUpdate::QuestionAsking => (false, false),
        StateUpdate::QuaWaiting => (false, false),
        StateUpdate::QuaQueue => (true, false),
        StateUpdate::QuaAnswer => (false, true),
        StateUpdate::QuestionAnswer => (false, false),
    };

    cx.render(rsx! {
        div {
            game_button {
                hidden: hidden,
                disabled: disabled,
                text: "skip",
                color: "accent-bg-yellow",
                event: ClientMessage::Input(InputEvent::Timeout)
            }
        }
    })
}

