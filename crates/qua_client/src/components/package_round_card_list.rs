use dioxus::prelude::*;
use qua_package::package_config::*;
use qua_game::prelude::*;

use crate::*;

pub fn package_round_card_list(cx: Scope) -> Element {
    let config = use_shared_state::<PackageConfig>(cx).unwrap();

    let add_round = move |_| {
        to_owned!(config);

        cx.spawn({
            async move {
                let mut config = config.write();
                config.rounds.push(RoundData {
                    name: "New Round".into(),
                    themes: vec![],
                });
            }
        });
    };

    cx.render(rsx! {
        div { class: "package-card-list",
            for (idx , round) in config.read().rounds.iter().enumerate() {
                package_round_card { round: Round::Normal(idx.into()), name: round.name.clone() }
            }
            div { onclick: add_round, class: "neutral-btn package-add-card", "Add Round" }
        }
    })
}
