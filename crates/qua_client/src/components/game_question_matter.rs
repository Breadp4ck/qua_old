use crate::{
    contexts::package_resource::{PackageResourceItem, ResourceUrlContent},
    *,
};
use dioxus::prelude::*;
use fermi::use_read;
use qua_game::prelude::*;

#[derive(PartialEq, Eq, Clone)]
pub enum MediaSource {
    Question,
    Answer,
}

#[derive(PartialEq, Props)]
pub struct GameQuestionMatterProps {
    question: Question,
}

pub fn game_question_matter(cx: Scope<GameQuestionMatterProps>) -> Element {
    let package = use_read(cx, PACKAGE_RESOURCE);
    let resource_item = use_ref(cx, || None::<PackageResourceItem>);

    let question = cx.props.question;

    let resource_load = use_future(cx, (), |_| {
        to_owned!(question, package, resource_item);

        async move {
            let package = package.unwrap().clone();
            let package = package.lock().await;

            let item = package.get(question);
            resource_item.set(Some(item));
        }
    });

    if let Some(item) = &*resource_item.read() {
        cx.render(rsx! { div { class: "message", "{item.title}" } })
    } else {
        cx.render(rsx! { div { "..." } })
    }
}

