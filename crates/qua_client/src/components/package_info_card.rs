use std::path::Path;

use crate::*;
use dioxus::prelude::*;
use qua_game::prelude::*;
use qua_package::package_config::*;

pub fn package_info_card(cx: Scope) -> Element {
    let config = use_shared_state::<PackageConfig>(cx).unwrap();

    let change_package_name = move |name: String| {
        to_owned!(config);

        cx.spawn({
            async move {
                let mut config = config.write();
                config.info.name = Some(name);
            }
        });
    };

    let change_package_author = move |author: String| {
        to_owned!(config);

        cx.spawn({
            async move {
                let mut config = config.write();
                config.info.author = Some(author);
            }
        });
    };

    let name = if let Some(name) = config.read().info.name.clone() {
        name
    } else {
        "".into()
    };

    let author = if let Some(author) = config.read().info.author.clone() {
        author
    } else {
        "".into()
    };

    cx.render(rsx! {
        div { class: "package-card-list",
            form { class: "package-card",
                div { class: "header",
                    div {
                        class: "text",
                        "General Info"
                    }
                }
                div {
                    class: "body",
                    div {
                        class: "row",
                        div {
                            "name:"
                        }
                        input {
                            onchange: move |event| {
                                change_package_name(event.data.value.clone())
                            },
                            class: "text-edit",
                            r#type: "text",
                            placeholder: "Write package title",
                            value: "{name}"
                        }
                    }
                    div {
                        class: "row",
                        div {
                            "author:"
                        }
                        input {
                            onchange: move |event| {
                                change_package_author(event.data.value.clone())
                            },
                            class: "text-edit",
                            r#type: "text",
                            placeholder: "Write your name",
                            value: "{author}"
                        }
                    }
                }
            }
        }
    })
}

