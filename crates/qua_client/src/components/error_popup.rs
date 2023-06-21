use dioxus::prelude::*;

pub struct ErrorMessage(pub Option<String>);

pub fn error_popup(cx: Scope) -> Element {
    let popup_message = use_shared_state::<ErrorMessage>(cx).unwrap();

    if let Some(message) = &popup_message.read().0 {
        render! {
            div { class: "error-popup menu",
                form {
                    prevent_default: "onsubmit",
                    div { class: "top",
                        div {
                            label { class: "accent-fg-red", r#for: "package", "Error: {message}" }
                        }
                    }
                    div { class: "bottom accent-bg-btn-red", input { r#type: "submit", value: "Yes?" } }
                }
            }
        }
    } else {
        render! { div {} }
    }
}
