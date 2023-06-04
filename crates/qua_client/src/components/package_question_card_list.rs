use dioxus::prelude::*;
use qua_game::prelude::*;
use qua_package::package_config::*;

use crate::*;

pub fn package_question_card_list(cx: Scope) -> Element {
    let config = use_shared_state::<PackageConfig>(cx).unwrap();
    let selected_theme = use_shared_state::<SelectedTheme>(cx).unwrap();

    if let Some(theme) = &*selected_theme.read() {
        match *theme {
            Theme::Normal(round_idx, theme_idx) => {
                let add_question = move |_| {
                    to_owned!(config);

                    cx.spawn({
                        async move {
                            let mut config = config.write();
                            config.rounds[round_idx].themes[theme_idx].items.push(ItemData {
                                cost: 100,
                                title: "".into(),
                                answer: "".into(),
                                question_content: QuestionContent::Empty,
                                question_description: None,
                                answer_content: AnswerContent::Empty,
                                answer_description: None,
                            });
                        }
                    });
                };

                cx.render(rsx! {
                    div { class: "package-card-list",
                        for (question_idx, _) in config.read().rounds[round_idx].themes[theme_idx].items.iter().enumerate() {
                            package_question_card { question: Question::Normal(round_idx, theme_idx, question_idx )}
                        }
                        div { onclick: add_question, class: "neutral-btn package-add-card", "Add Question" }
                    }
                })
            },
        }
    } else {
        cx.render(rsx! {
            div { class: "package-card-list",
                "EMPYT"
            }
        })
    }
}
