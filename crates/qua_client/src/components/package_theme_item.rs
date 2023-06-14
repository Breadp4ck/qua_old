use dioxus::prelude::*;
use qua_game::prelude::*;
use qua_package::package_config::*;

use super::package_editor::SelectedTheme;

#[derive(PartialEq, Props)]
pub struct PackageThemeItemProps {
    name: String,
    theme: Theme,
}

pub fn package_theme_item(cx: Scope<PackageThemeItemProps>) -> Element {
    let config = use_shared_state::<PackageConfig>(cx).unwrap();
    let selected_theme = use_shared_state::<SelectedTheme>(cx).unwrap();
    let theme = cx.props.theme;

    let selected = if let Some(removed_theme) = &*selected_theme.read() {
        if *removed_theme == theme {
            true
        } else {
            false
        }
    } else {
        false
    };

    let change_theme_name = move |name| {
        to_owned!(config);

        cx.spawn({
            async move {
                let mut config = config.write();
                match theme {
                    Theme::Normal(round_idx, theme_idx) => {
                        config.rounds[round_idx].themes[theme_idx].name = name;
                    }
                }
            }
        });
    };

    let edit_theme = move |_| {
        to_owned!(config, selected_theme);

        cx.spawn({
            async move {
                let mut selected_theme = selected_theme.write();
                *selected_theme = Some(theme.clone());
            }
        });
    };

    let remove_theme = move |_| {
        to_owned!(config, selected_theme);

        cx.spawn({
            async move {
                let mut config = config.write();
                match theme {
                    Theme::Normal(round_idx, theme_idx) => {
                        if selected {
                            let mut selected_theme = selected_theme.write();
                            *selected_theme = None;
                        }

                        config.rounds[round_idx].themes.remove(theme_idx);
                    }
                }
            }
        });
    };

    let selected_style = if selected {
        "color:greenyellow;"
    } else {
        ""
    };

    let total_questions = match theme {
        Theme::Normal(round_idx, theme_idx) => {
            config.read().rounds[round_idx].themes[theme_idx].items.len()
        }
    };

    cx.render(rsx! {
        div {
            class: "package-theme-item",
            input {
                onchange: move |event| {
                    change_theme_name(event.data.value.clone());
                },
                style: "{selected_style}",
                class: "text-edit",
                r#type: "text",
                value: "{cx.props.name}"
            }
            div {
                class: "theme-count",
                "({total_questions})"
            }
            div {
                onclick: edit_theme,
                class: "theme-edit neutral-btn",
                "edit"
            }
            div {
                class: "theme-remove",
                onclick: remove_theme,
                "x"
            }
        }
    })
}
