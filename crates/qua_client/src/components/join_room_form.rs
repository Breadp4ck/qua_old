use std::sync::Arc;

use crate::*;
use dioxus::prelude::*;
use dioxus_router::*;
use fermi::use_set;
use qua_game::person::prelude::*;
use tokio::sync::Mutex;

pub fn join_room_form(cx: Scope) -> Element {
    let set_ticket = use_set(cx, TICKET);
    let set_person_type = use_set(cx, PERSON_TYPE);
    let obtain_room_code = use_state(cx, || false);
    let set_room_code = use_set(cx, ROOM_CODE);
    let set_package_resource = use_set(cx, PACKAGE_RESOURCE);

    let join_room = move |username: String, room_code| {
        to_owned!(
            set_ticket,
            set_person_type,
            set_package_resource,
            set_room_code,
            obtain_room_code
        );

        cx.spawn({
            async move {
                let room_code = RoomCode::from(room_code);

                let ticket = RoomService::obtain_ticket(
                    Person::Player(Player::with_name(PersonName::new(&username))),
                    room_code.clone(),
                )
                .await;

                let archive_file = RoomService::get_room_package(&room_code).await;
                let package_resource = PackageResource::new(&archive_file);

                set_ticket(Some(ticket));
                set_person_type(PersonType::Player);
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
            class: "accent-focus-yellow",
            prevent_default: "onsubmit",
            onsubmit: move |event| join_room(
                event.data.values["username"][0].clone(),
                event.data.values["code"][0].clone(),
            ),
            div { class: "top",
                div { class: "input",
                    label { class: "accent-fg-yellow", "NAME" }
                    input {
                        class: "accent-focus-yellow",
                        r#type: "text",
                        name: "username",
                        placeholder: "Write your performing name"
                    }
                }
                div { class: "input",
                    label { class: "accent-fg-yellow", "ROOM CODE" }
                    input {
                        class: "accent-focus-yellow",
                        r#type: "text",
                        name: "code",
                        placeholder: "Write appropriate room code"
                    }
                }
            }
            div { class: "bottom accent-bg-btn-yellow", input { "type": "submit", value: "Join" } }
        }
    })
}

