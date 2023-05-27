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
pub struct GameMediaContentProps {
    question: Question,
    media_source: MediaSource,
}

pub fn game_media_content(cx: Scope<GameMediaContentProps>) -> Element {
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
                log::info!("Контент запихан!");
                // pub title: String,
                // pub cost: Scores,
                // pub question_url_content: ResourceUrlContent,
                // pub question_description: String,
                // pub answer_url_content: ResourceUrlContent,
                // pub answer_description: String,
            }
    });

    if let Some(item) = &*resource_item.read() {
        let url = match cx.props.media_source {
            MediaSource::Answer => item.answer_url_content.clone(),
            MediaSource::Question => item.question_url_content.clone(),
        };

        match url {
            ResourceUrlContent::Text { url } => cx.render(rsx! { div { "{url}" } }),
            ResourceUrlContent::Picture { url } => cx.render(rsx! {
                div {
                    video { src: "{url}" },
                }
            }),
            ResourceUrlContent::Video { url } => cx.render(rsx! {
                div {
                    video { src: "{url}", autoplay: "true" },
                }
            }),
            ResourceUrlContent::Sound { url } => cx.render(rsx! {
                div {
                    audio { src: "{url}", autoplay: "true" },
                }
            }),
            ResourceUrlContent::Empty => cx.render(rsx! { div { "ПУСТО" } }),
        }
    } else {
        cx.render(rsx! { div { "ЖДЁМ МЕДИ КОНТЕНТ" } })
    }
}
