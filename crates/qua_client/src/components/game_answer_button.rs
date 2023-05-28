use std::time::Duration;

use dioxus::prelude::*;
use qua_game::prelude::*;

use crate::*;

pub fn game_answer_button(cx: Scope) -> Element {
    let state = use_shared_state::<StateUpdate>(cx).unwrap();

    let (disabled, hidden) = match &*state.read() {
        StateUpdate::Init => (true, false),
        StateUpdate::Greet => (true, false),
        StateUpdate::Overview => (true, false),
        StateUpdate::Picking => (true, false),
        StateUpdate::QuestionAppearance => (true, false),
        StateUpdate::QuestionMatter => (true, false),
        StateUpdate::QuestionAsking => (true, false),
        StateUpdate::QuaWaiting => (false, false),
        StateUpdate::QuaQueue => (true, false),
        StateUpdate::QuaAnswer => (true, false),
        StateUpdate::QuestionAnswer => (true, false),
    };

    cx.render(rsx! {
        div { 
            game_button {
                hidden: hidden,
                disabled: disabled,
                text: "qua!",
                color: "accent-bg-yellow",
                event: ClientMessage::Input(InputEvent::Answer(Duration::from_secs(1))),
            }
        }
    })
}
