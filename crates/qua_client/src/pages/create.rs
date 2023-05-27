use crate::PACKAGE_RESOURCE;
use crate::contexts::package_resource::{PackageResource, ResourceUrlContent};
use crate::{services::prelude::*, ROOM_CODE, TICKET};
use dioxus::prelude::*;
use dioxus_router::*;
use fermi::prelude::*;
use qua_game::person::prelude::*;
use tokio::sync::Mutex;
use std::collections::HashMap;
use std::io::Read;
use std::{io::Cursor, sync::Arc};

pub fn create(cx: Scope) -> Element {
    let set_ticket = use_set(cx, TICKET);
    let set_room_code = use_set(cx, ROOM_CODE);
    let set_package_resource = use_set(cx, PACKAGE_RESOURCE);
    let obtain_room_code = use_state(cx, || false);
    let package: &UseRef<Vec<u8>> = use_ref(cx, Vec::new);

    let create_room = move |username: String| {
        to_owned!(package, set_ticket, set_room_code, set_package_resource, obtain_room_code);

        cx.spawn({
            async move {
                let archive_file: Vec<u8> = package.read().to_vec();
                let package_resource = PackageResource::new(&archive_file);

                let room_code = RoomService::create_room(archive_file).await;
                let ticket = RoomService::obtain_ticket(
                    Person::Host(Host::with_name(PersonName::new(&username))),
                    room_code.clone(),
                )
                .await;

                set_ticket(Some(ticket));
                set_room_code(Some(room_code));
                set_package_resource(Some(Arc::new(Mutex::new(package_resource))));

                obtain_room_code.set(true);
            }
        });
    };

    cx.render(rsx! {
        div { class: "center-screen",
            if *obtain_room_code.get() {
                rsx! { Redirect { to: "/room" } }
            }
            form {
                class: "join-game",
                prevent_default: "onsubmit",
                onsubmit: move |event| {
                    log::info!("{:?}", event.data);
                    create_room(event.data.values["username"][0].clone());
                },
                div { class: "join-game-input",
                    label { "PACKAGE" }
                    input {
                        r#type: "file",
                        accept: ".zip, .qua, .jpg, .png, .txt",
                        name: "package",
                        onchange: |evt| {
                            to_owned![package];
                            async move {
                                if let Some(file_engine) = &evt.files {
                                    let files = file_engine.files();
                                    for file_name in &files {
                                        if let Some(file) = file_engine.read_file(file_name).await {
                                            *package.write() = file;
                                            log::info!("Loaded: {:?}", file_name);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                div { class: "join-game-input",
                    label { "NAME" }
                    input { "type": "text", name: "username" }
                }
                div { class: "join-game-input", input { "type": "submit", value: "Create" } }
            }
        }
    })
}
