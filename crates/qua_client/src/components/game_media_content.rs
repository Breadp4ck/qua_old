use crate::{
    contexts::package_resource::{PackageResourceItem, ResourceContent},
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
            (PersonType::Host, _) => set_info(format!("Answer: {} | Question: {}", item.answer, item.title).into()),
            (_, MediaSource::Answer) => set_info(format!("Answer: {}", item.answer).into()),
            _ => (),
        }

        let (general, resource_content) = match cx.props.media_source {
            MediaSource::Answer => (item.answer.clone(), item.answer_url_content.clone()),
            MediaSource::Question => (item.title.clone(), item.question_url_content.clone()),
        };

        match resource_content {
            ResourceContent::Text { text } => cx.render(rsx! { div { class: "message", "{text}" } }),
            ResourceContent::Picture { url } => cx.render(rsx! {
                div { class: "media-content", img { src: "{url}" } }
            }),
            ResourceContent::Video { url } => cx.render(rsx! {
                div { class: "media-content", video { src: "{url}", autoplay: "true" } }
            }),
            ResourceContent::Sound { url } => cx.render(rsx! {
                div {
                    class: "audio",
                    audio { src: "{url}", autoplay: "true" },
                    img {
                        src: "/assets/icons/music-outline.svg",
                        alt: "MUSIC"
                    }
                }
            }),
            ResourceContent::Empty => cx.render(rsx! { div { class: "message", "{general}" } }),
        }
    } else {
        cx.render(rsx! { div { "..." } })
    }
}
