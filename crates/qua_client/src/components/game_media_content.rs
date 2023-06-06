use crate::{
    contexts::package_resource::{PackageResourceItem, ResourceUrlContent},
    *,
};
use dioxus::prelude::*;
use fermi::use_read;
use qua_game::prelude::*;

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum MediaSource {
    Question,
    Answer,
}

#[derive(PartialEq, Props)]
pub struct GameMediaContentProps {
    question: Question,
    media_source: MediaSource,
}

pub fn game_media_content(cx: Scope<GameMediaContentProps>) -> Element {
    let package = use_read(cx, PACKAGE_RESOURCE);
    let resource_item = use_ref(cx, || None::<PackageResourceItem>);
    let set_info = use_set(cx, INFO);
    let person_type = use_read(cx, PERSON_TYPE);

    let question = cx.props.question;
    let media_source = cx.props.media_source;

    let resource_load = use_future(cx, (), |_| {
        to_owned!(question, package, resource_item, set_info, person_type);

        async move {
            let package = package.unwrap().clone();
            let package = package.lock().await;

            let item = package.get(question);

            resource_item.set(Some(item));
        }
    });


    if let Some(item) = &*resource_item.read() {
        match (person_type, media_source) {
            (PersonType::Host, _) => set_info(format!("Answer: {}", item.answer).into()),
            (_, MediaSource::Answer) => set_info(format!("Answer: {}", item.answer).into()),
            _ => (),
        }

        let (general, url) = match cx.props.media_source {
            MediaSource::Answer => (item.answer.clone(), item.answer_url_content.clone()),
            MediaSource::Question => (item.title.clone(), item.question_url_content.clone()),
        };

        match url {
            ResourceUrlContent::Text { url } => cx.render(rsx! { div { "{url}" } }),
            ResourceUrlContent::Picture { url } => cx.render(rsx! {
                div { class: "media-content", img { src: "{url}" } }
            }),
            ResourceUrlContent::Video { url } => cx.render(rsx! {
                div { class: "media-content", video { src: "{url}", autoplay: "true" } }
            }),
            ResourceUrlContent::Sound { url } => cx.render(rsx! {
                div { audio { src: "{url}", autoplay: "true" } }
            }),
            ResourceUrlContent::Empty => cx.render(rsx! { div { class: "message", "{general}" } }),
        }
    } else {
        cx.render(rsx! { div { "..." } })
    }
}
