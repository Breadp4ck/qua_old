use crate::services::prelude::*;
use crate::{GameWsReceiver, GameWsSender};
use ewebsock::{WsReceiver, WsSender};
use dioxus::prelude::*;
use dioxus_router::*;
use qua_game::person::prelude::*;

pub fn create(cx: Scope) -> Element {
    let obtain_room_code = use_state(cx, || false);

    let ws_sender = use_shared_state::<GameWsSender>(cx).unwrap();
    let ws_receiver = use_shared_state::<GameWsReceiver>(cx).unwrap();

    let (sender, receiver) = std::sync::mpsc::channel::<(WsSender, WsReceiver)>();

    let create_room = move |username: String| {
        let obtain_room_code = obtain_room_code.to_owned();
        let sender = sender.to_owned();

        cx.spawn({
            async move {
                let (ws_sender, ws_receiver) = RoomService::join_room(
                    RoomService::obtain_ticket(
                        Person::Host(Host::with_name(PersonName::new(&username))),
                        RoomService::create_room("kek".to_string()).await,
                    )
                    .await,
                )
                .await;

                sender.send((ws_sender, ws_receiver)).expect("Failed to send");
                obtain_room_code.set(true);
            }
        });
    };

    cx.render(rsx! {
        div {
            class: "center-screen",
            if *obtain_room_code.get() {
                let (sender, receiver) = receiver.recv().unwrap();
                ws_sender.write_silent().0 = Some(sender);
                ws_receiver.write_silent().0 = Some(receiver);

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
