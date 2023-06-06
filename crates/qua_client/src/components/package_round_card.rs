use crate::*;
use dioxus::prelude::*;
use qua_game::prelude::*;
use qua_package::package_config::*;

#[derive(PartialEq, Props)]
pub struct PackageRoundCardProps {
    name: String,
    round: Round,
}

pub fn package_round_card(cx: Scope<PackageRoundCardProps>) -> Element {
    let config = use_shared_state::<PackageConfig>(cx).unwrap();
    let round = cx.props.round;
    let selected_theme = use_shared_state::<SelectedTheme>(cx).unwrap();

    let selected = if let Some(removed_theme) = &*selected_theme.read() {
        if let Round::Normal(round_idx) = round {
            match removed_theme {
                Theme::Normal(selected_round_idx, _) => {
                    if *selected_round_idx == round_idx {
                        true
                    } else {
                        false
                    }
                }
                _ => false,
            }
        } else {
            false
        }
    } else {
        false
    };

    let change_round_name = move |name| {
        to_owned!(config);

        cx.spawn({
            async move {
                let mut config = config.write();
                match round {
                    Round::Normal(round_idx) => config.rounds[round_idx].name = name,
                    Round::Final => todo!(),
                }
            }
        });
    };

    let add_theme = move |_| {
        to_owned!(config);

        cx.spawn({
            async move {
                let mut config = config.write();
                match round {
                    Round::Normal(round_idx) => config.rounds[round_idx].themes.push(ThemeData {
                        name: "New Theme".into(),
                        items: vec![],
                    }),
                    Round::Final => todo!(),
                }
            }
        });
    };

    let remove_round = move |_| {
        to_owned!(config, selected_theme);

        cx.spawn({
            async move {
                let mut config = config.write();
                match round {
                    Round::Normal(round_idx) => {
                        config.rounds.remove(round_idx);
                    }
                    Round::Final => todo!(),
                }
                if selected {
                    *selected_theme.write() = None;
                }
            }
        });
    };

    cx.render(rsx! {
        form { class: "package-card",
            div { class: "header",
                input {
                    onchange: move |event| {
                        change_round_name(event.data.value.clone());
                    },
                    class: "text-edit",
                    r#type: "text",
                    value: "{cx.props.name}"
                }
                div { class: "remove neutral-btn",
                    onclick: remove_round,
                    "x"
                }
            }
            div {
                class: "body",
                match round {
                    Round::Normal(round_idx) => {
                        if config.read().rounds[round_idx].themes.is_empty() {
                            rsx! { div { class: "no-themes", "There are no themes" } }
                        } else {
                            rsx!{
                                ul {
                                    for (theme_idx , theme) in config.read().rounds[round_idx].themes.iter().enumerate() {
                                        li {
                                            package_theme_item {
                                                name: theme.name.clone(),
                                                theme: Theme::Normal(round_idx, theme_idx),
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    Round::Final => todo!(),
                }
                div {
                    div {
                        onclick: add_theme,
                        class: "add-theme neutral-btn",
                        "Add Theme"
                    }
                }
            }
        }
    })
}
