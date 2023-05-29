use crate::*;
use dioxus::prelude::*;
use dioxus_router::*;
use fermi::prelude::*;
use qua_game::person::prelude::*;
use std::sync::Arc;
use tokio::sync::Mutex;

pub fn create_room_form(cx: Scope) -> Element {
    let set_ticket = use_set(cx, TICKET);
    let set_person_type = use_set(cx, PERSON_TYPE);
    let set_room_code = use_set(cx, ROOM_CODE);
    let set_package_resource = use_set(cx, PACKAGE_RESOURCE);
    let obtain_room_code = use_state(cx, || false);
    let package: &UseRef<Vec<u8>> = use_ref(cx, Vec::new);

    let create_room = move |username: String| {
        to_owned!(
            package,
            set_ticket,
            set_person_type,
            set_room_code,
            set_package_resource,
            obtain_room_code
        );

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
                set_person_type(PersonType::Host);
                set_room_code(Some(room_code));
                set_package_resource(Some(Arc::new(Mutex::new(package_resource))));

                obtain_room_code.set(true);
            }
        });
    };

    cx.render(rsx! {
        if *obtain_room_code.get() {
            rsx! { Redirect { to: "/room" } }
        }
        form {
            prevent_default: "onsubmit",
            onsubmit: move |event| {
                log::info!("{:?}", event.data);
                create_room(event.data.values["username"][0].clone());
            },
            div { class: "top",
                div {
                    label { class: "accent-fg-red", r#for: "package", "PACKAGE" }
                    input {
                        class: "accent-focus-red",
                        r#type: "file",
                        accept: ".zip, .qua",
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
                div {
                    label { class: "accent-fg-red", r#for: "package", "NAME" }
                    input {
                        class: "accent-focus-red",
                        r#type: "text",
                        name: "username",
                        placeholder: "Write your performing name"
                    }
                }
            }
            div { class: "bottom accent-bg-btn-red", input { r#type: "submit", value: "Create" } }
        }
    })
}
