use dioxus::prelude::*;
use qua_game::prelude::*;

use crate::*;

enum ProgressState {
    Leak { secs: f32 },
    Freeze { percent: f32 },
    Nothing,
}

pub fn game_progress_bar(cx: Scope) -> Element {
    let state = use_shared_state::<StateUpdate>(cx).unwrap();

    let progress = match &*state.read() {
        StateUpdate::Init => ProgressState::Nothing,
        StateUpdate::Greet => ProgressState::Nothing,
        StateUpdate::Overview => ProgressState::Nothing,
        StateUpdate::RoundPreview => ProgressState::Nothing,
        StateUpdate::Picking => ProgressState::Nothing,
        StateUpdate::QuestionAppearance => ProgressState::Nothing,
        StateUpdate::QuestionMatter => ProgressState::Nothing,
        StateUpdate::QuestionAsking => ProgressState::Nothing,
        StateUpdate::QuaWaiting => ProgressState::Leak { secs: 10.0 },
        StateUpdate::QuaQueue => ProgressState::Nothing,
        StateUpdate::QuaAnswer => ProgressState::Leak { secs: 10.0 },
        StateUpdate::QuestionAnswer => ProgressState::Nothing,
        StateUpdate::Ending => ProgressState::Nothing,
    };

    match progress {
        ProgressState::Leak { secs } => cx.render(rsx! {
            div { class: "timer",
                span { style: "width:100%;", span { style: "animation-duration: {secs}s;", class: "progress" } }
            }
        }),
        ProgressState::Freeze { percent } => cx.render(rsx! {
            div { class: "timer",
                span { style: "width:100%;", span { style: "width:{percent}%", class: "progress-wait" } }
            }
        }),
        ProgressState::Nothing => cx.render(rsx! {
            div { class: "timer",
                span { style: "width:100%;", span { class: "progress-wait" } }
            }
        }),
    }
}
