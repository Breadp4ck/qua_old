use crate::{
    contexts::package_resource::{PackageResourceItem, ResourceUrlContent},
    *,
};
use dioxus::prelude::*;
use fermi::use_read;
use qua_game::prelude::*;

#[derive(PartialEq, Props)]
pub struct GameBoardThemeProps {
    theme: Theme,
}

pub fn game_board_theme(cx: Scope<GameBoardThemeProps>) -> Element {
    let package = use_read(cx, PACKAGE_RESOURCE);
    let title_item = use_ref(cx, || None::<String>);

    let theme = cx.props.theme;

    let resource_load = use_future(cx, (), |_| {
        to_owned!(theme, package, title_item);

        async move {
            let package = package.unwrap().clone();
            let package = package.lock().await;

            let title = package.get_theme(theme);
            title_item.set(Some(title));
        }
    });

    if let Some(title) = &*title_item.read() {
        cx.render(rsx! { div { class: "title", "{title}" } })
    } else {
        cx.render(rsx! { div { ":/" } })
    }
}


