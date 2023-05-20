use crate::{services::prelude::*, TICKET};
use dioxus::prelude::*;
use dioxus_router::*;
use fermi::use_set;
use qua_game::person::prelude::*;

pub fn join(cx: Scope) -> Element {
    let set_ticket = use_set(cx, TICKET);
    let obtain_room_code = use_state(cx, || false);

    let join_room = move |username: String, room_code| {
        to_owned!(set_ticket, obtain_room_code);

        cx.spawn({
            async move {
                set_ticket(Some(
                    RoomService::obtain_ticket(
                        Person::Player(Player::with_name(PersonName::new(&username))),
                        RoomCode::from(room_code),
                    )
                    .await,
                ));

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
