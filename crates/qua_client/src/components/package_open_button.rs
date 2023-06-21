use crate::*;
use dioxus::prelude::*;
use qua_package::package_config::*;
use wasm_bindgen::JsCast;

pub fn package_open_button(cx: Scope) -> Element {
    let config = use_shared_state::<PackageConfig>(cx).unwrap();
    let questions = use_shared_state::<QuestionsData>(cx).unwrap();
    let answers = use_shared_state::<AnswersData>(cx).unwrap();

    cx.render(rsx!{
        input {
            id: "package-editor-open-handler",
            style: "display: none",
            class: "accent-focus-red",
            r#type: "file",
            accept: ".qua",
            name: "package",
            onchange: |evt| {
                to_owned![config, questions, answers];

                async move {
                    if let Some(file_engine) = &evt.files {
                        let files = file_engine.files();
                        for file_name in &files {
                            if let Some(file) = file_engine.read_file(file_name).await {
                                let (new_config, new_questions, new_answers) = PackageResource::new_as_parts(&file);

                                *config.write() = new_config;
                                questions.write().0 = new_questions;
                                answers.write().0 = new_answers;

                                log::info!("Loaded: {:?}", file_name);
                            }
                        }
                    }
                }
            }
        },
        div {
            class: "game-button accent-bg-btn-blue",
            onclick: move |_| {
                let element = web_sys::window().unwrap().document().unwrap().get_element_by_id("package-editor-open-handler").unwrap();
                let html_element = element.dyn_into::<web_sys::HtmlElement>().unwrap();
                html_element.click();
            },
            "Load"
        }
    })
}

