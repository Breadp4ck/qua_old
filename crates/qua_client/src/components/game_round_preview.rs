use crate::{
    contexts::package_resource::{PackageResourceItem, ResourceContent},
    *,
};
use dioxus::prelude::*;
use fermi::use_read;
use qua_game::prelude::*;

#[derive(PartialEq, Props)]
pub struct GameRoundPreviewProps {
    round: Round,
}

pub fn game_round_preview(cx: Scope<GameRoundPreviewProps>) -> Element {
    let package = use_read(cx, PACKAGE_RESOURCE);
    let round_name = use_ref(cx, || None::<String>);
    let set_info = use_set(cx, INFO);

    let round = cx.props.round;

    let resource_load = use_future(cx, (), |_| {
        to_owned!(round, package, round_name, set_info);

        async move {
            let package = package.unwrap().clone();
            let package = package.lock().await;

            let name = package.get_round(round);
            set_info(format!("Round: {}", &name).into());
            round_name.set(Some(name));
        }
    });

    if let Some(name) = &*round_name.read() {
        cx.render(rsx! { div { class: "message", "{name}" } })
    } else {
        cx.render(rsx! { div { "..." } })
    }
}


