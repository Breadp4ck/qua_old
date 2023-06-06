use crate::*;
use dioxus::prelude::*;
// use qua_package::package_config::*;

pub fn package_open_button(cx: Scope) -> Element {
    // let config = use_shared_state::<PackageConfig>(cx).unwrap();
    // let questions = use_shared_state::<QuestionsData>(cx).unwrap();
    // let answers = use_shared_state::<AnswersData>(cx).unwrap();

    cx.render(rsx!{
        div {
            class: "game-button accent-bg-btn-blue",
            "Load"
        }
    })
}

