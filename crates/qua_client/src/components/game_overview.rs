use crate::{
    contexts::package_resource::{PackageResourceItem, ResourceContent},
    *,
};
use dioxus::prelude::*;
use fermi::use_read;
use qua_game::prelude::*;

pub fn game_overview(cx: Scope) -> Element {
    let package = use_read(cx, PACKAGE_RESOURCE);
    let title_author = use_ref(cx, || None::<(String, String)>);
    let set_info = use_set(cx, INFO);

    let resource_load = use_future(cx, (), |_| {
        to_owned!(package, title_author, set_info);

        async move {
            let package = package.unwrap().clone();
            let package = package.lock().await;

            let info = package.get_info();

            let title = if let Some(title) = info.name {
                title
            } else {
                "--".into()
            };
            let author = if let Some(author) = info.author {
                author
            } else {
                "--".into()
            };

            title_author.set(Some((title, author)));
        }
    });

    if let Some((title, author)) = &*title_author.read() {
        cx.render(rsx! { div { class: "message", p {"Package: {title.clone()}"}, p {"Author: {author.clone()}"} } })
    } else {
        cx.render(rsx! { div { "..." } })
    }
}
