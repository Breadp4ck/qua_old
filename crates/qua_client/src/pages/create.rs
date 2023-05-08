use crate::services::prelude::*;
use dioxus::prelude::*;
use dioxus_router::*;
use qua_game::person::prelude::*;

pub fn create(cx: Scope) -> Element {
    let obtain_room_code = use_state(cx, || false);

    let create_room = move |username: String| {
        let obtain_room_code = obtain_room_code.to_owned();

        cx.spawn({
            async move {
                RoomService::join_room(
                    RoomService::obtain_ticket(
                        Person::Host(Host::with_name(PersonName::new(&username))),
                        // RoomService::create_room("kek".to_string()).await,
                        RoomCode { data: "lol".to_string() }
                    )
                    .await,
                )
                .await;

                obtain_room_code.set(true);
            }
        });
    };

    cx.render(rsx! {
        div {
            class: "center-screen",
            if *obtain_room_code.get() {
                rsx! { Redirect { to: "/game" } }
            },
            form {
                class: "join-game",
                prevent_default: "onsubmit",
                onsubmit: move |event| create_room(event.data.values["username"].clone()),
                // div {
                //     class: "join-game-input",
                //     label {
                //         "PACKAGE"
                //     }
                //     input {
                //         "type": "file",
                //         name: "package"
                //     }
                // }
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
