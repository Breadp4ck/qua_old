use std::sync::Arc;

use crate::{services::prelude::*, TICKET, PACKAGE_RESOURCE, contexts::package_resource::PackageResource};
use dioxus::prelude::*;
use dioxus_router::*;
use fermi::use_set;
use qua_game::person::prelude::*;
use tokio::sync::Mutex;

pub fn join(cx: Scope) -> Element {
    let set_ticket = use_set(cx, TICKET);
    let obtain_room_code = use_state(cx, || false);
    let set_package_resource = use_set(cx, PACKAGE_RESOURCE);

    let join_room = move |username: String, room_code| {
        to_owned!(set_ticket, set_package_resource, obtain_room_code);

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

                let item = package_resource.get(qua_game::game::QuestionType::Normal(1.into(), 1.into(), 1.into()));
                log::info!("{:?}", item.title);

                set_ticket(Some(ticket));
                set_package_resource(Some(Arc::new(Mutex::new(package_resource))));

                obtain_room_code.set(true);
            }
        });
    };

    cx.render(rsx! {
        div {
            class: "center-screen",
            if *obtain_room_code.get() {
                rsx! { Redirect { to: "/room" } }
            },
            form {
                class: "join-game",
                prevent_default: "onsubmit",
                onsubmit: move |event| join_room(
                    event.data.values["username"][0].clone(),
                    event.data.values["code"][0].clone()
                ),
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
                    label {
                        "ROOM CODE"
                    }
                    input {
                        "type": "text",
                        name: "code"
                    }
                }
                div {
                    class: "join-game-input",
                    input {
                        "type": "submit",
                        value: "Join"
                    }
                }
            }

        }
    })
}
