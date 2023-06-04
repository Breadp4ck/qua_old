use std::collections::HashMap;

use crate::components::prelude::*;
use dioxus::prelude::*;
use qua_game::prelude::{Theme, Question};
use qua_package::package_config::*;

pub type SelectedTheme = Option<Theme>;
pub type QuestionsData = HashMap<Question, Vec<u8>>;
pub type AnswersData = HashMap<Question, Vec<u8>>;

pub fn package_editor(cx: Scope) -> Element {
    use_shared_state_provider::<PackageConfig>(cx, || PackageConfig::default());
    use_shared_state_provider::<SelectedTheme>(cx, || None);
    use_shared_state_provider::<QuestionsData>(cx, || HashMap::new());
    use_shared_state_provider::<AnswersData>(cx, || HashMap::new());

    cx.render(rsx! {
        div{
            class: "center-screen",
            div { class: "package-editor",
                div { class: "rounds", package_round_card_list {} }
                div { class: "questions", package_question_card_list {} }
                div { class: "general" }
            }
        }
    })
}
