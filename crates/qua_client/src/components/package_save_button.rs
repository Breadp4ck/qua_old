use crate::*;
use dioxus::prelude::*;
use qua_package::package_config::*;
use wasm_bindgen::JsCast;

pub fn package_save_button(cx: Scope) -> Element {
    let config = use_shared_state::<PackageConfig>(cx).unwrap();
    let questions = use_shared_state::<QuestionsData>(cx).unwrap();
    let answers = use_shared_state::<AnswersData>(cx).unwrap();

    cx.render(rsx!{
        a { // used for download file
            id: "package-blob-url",
        },
        div {
            class: "game-button accent-bg-btn-green",
            onclick: move |_| {
                to_owned!(config, questions, answers);

                let data = PackageResource::export(&*config.read(), &*questions.read(), &*answers.read());
                let uint8arr = unsafe { js_sys::Uint8Array::view(&data) };
                let array = js_sys::Array::new();
                array.push(&uint8arr);

                let blob = web_sys::Blob::new_with_u8_array_sequence(&array).unwrap();
                let url = web_sys::Url::create_object_url_with_blob(&blob).unwrap();

                let element = web_sys::window().unwrap().document().unwrap().get_element_by_id("package-blob-url").unwrap();
                element.set_attribute("href", &url).unwrap();
                element.set_attribute("download", "pack.qua").unwrap();

                let html_element = element.dyn_into::<web_sys::HtmlElement>().unwrap();
                html_element.click();
            },
            "Save"
        }
    })
}
