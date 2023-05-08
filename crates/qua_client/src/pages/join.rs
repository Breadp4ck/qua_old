use crate::services::prelude::*;
use dioxus::prelude::*;
use dioxus_router::*;
use qua_game::person::prelude::*;

pub fn join(cx: Scope) -> Element {
    let obtain_room_code = use_state(cx, || false);

    let join_room = move |username: String, room_code| {
        let obtain_room_code = obtain_room_code.to_owned();

        cx.spawn({
            async move {
                RoomService::join_room(
                    RoomService::obtain_ticket(
                        Person::Player(Player::with_name(PersonName::new(&username))),
                        RoomCode::from(room_code),
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
                onsubmit: move |event| join_room(
                    event.data.values["username"].clone(),
                    event.data.values["code"].clone()
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
                        "ROOM*CODE"
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
