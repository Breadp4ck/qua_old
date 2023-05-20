use crate::{services::prelude::*, ROOM_CODE, TICKET};
use dioxus::prelude::*;
use dioxus_router::*;
use fermi::prelude::*;
use qua_game::person::prelude::*;
use qua_package::package_resource::PackageResource;
use std::collections::HashMap;
use std::io::Read;
use std::{io::Cursor, sync::Arc};

pub fn create(cx: Scope) -> Element {
    let set_ticket = use_set(cx, TICKET);
    let set_room_code = use_set(cx, ROOM_CODE);
    let obtain_room_code = use_state(cx, || false);
    let blob_url = use_state(cx, || String::new());
    let package: &UseRef<Vec<u8>> = use_ref(cx, Vec::new);

    let create_room = move |username: String| {
        to_owned!(package, set_ticket, set_room_code, blob_url, obtain_room_code);

        cx.spawn({
            async move {
                let buf: Vec<u8> = package.read().to_vec();
                let mut zip = zip::ZipArchive::new(Cursor::new(buf)).unwrap();

                let mut data_string = String::new();
                let mut data_bin = Vec::new();

                for i in 0..zip.len() {
                    let mut file = zip.by_index(i).unwrap();

                    if file.is_dir() {
                        continue;
                    }

                    if file.name() == "Pack.toml" {
                        file.read_to_string(&mut data_string).unwrap();
                        let package_resource = PackageResource::from_toml(data_string.as_str());
                    } else {
                        file.read_to_end(&mut data_bin).unwrap();

                        let uint8arr = unsafe { js_sys::Uint8Array::view(&data_bin) };
                        let array = js_sys::Array::new();
                        array.push(&uint8arr);

                        let mut properties = web_sys::BlobPropertyBag::new();
                        properties.type_("video/webm");

                        let blob = web_sys::Blob::new_with_u8_array_sequence_and_options(&array, &properties).unwrap();
                        let url_object = web_sys::Url::create_object_url_with_blob(&blob).unwrap();
                        blob_url.set(url_object);
                    }

                }

                let room_code = RoomService::create_room().await;

                set_ticket(Some(
                    RoomService::obtain_ticket(
                        Person::Host(Host::with_name(PersonName::new(&username))),
                        room_code.clone(),
                    )
                    .await,
                ));

                set_room_code(Some(room_code));

                obtain_room_code.set(true);
            }
        });
    };

    cx.render(rsx! {
        div {
            class: "center-screen",
            if *obtain_room_code.get() {
                // rsx! { Redirect { to: "/room" } }
                rsx! {
                    video {
                        src: "{blob_url.get()}",
                        autoplay: true,
                    },
                }
            },
            form {
                class: "join-game",
                prevent_default: "onsubmit",
                onsubmit: move |event| {
                    log::info!("{:?}", event.data);
                    create_room(
                        event.data.values["username"][0].clone()
                ); },
                div {
                    class: "join-game-input",
                    label {
                        "PACKAGE"
                    }
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
                                        if let Some(file) = file_engine.read_file(file_name).await{
                                            *package.write() = file;
                                            log::info!("Loaded: {:?}", file_name);
                                        }
                                    }
                                }
                            }
                        },
                    }
                }
                div {
                    class: "join-game-input",
                    label {
                        "NAME"
                    }
                    input {
                       "type": "text",
                        name: "username"
                    }
                }
                div {
                    class: "join-game-input",
                    input {
                        "type": "submit",
                        value: "Create"
                    }
                }
            }

        }
    })
}
