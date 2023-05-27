use crate::Connection;
use dioxus::prelude::*;
use qua_game::prelude::*;

#[derive(PartialEq, Props)]
pub struct GameBoardQuestionButtonProps {
    cost: Scores,
    answered: bool,
    question: Question,
}

pub fn game_board_question_button(cx: Scope<GameBoardQuestionButtonProps>) -> Element {
    let maybe_connection = use_shared_state::<Connection>(cx).unwrap().write_silent();

    let client = if let Some(connection) = &*maybe_connection {
        Some(connection.clone())
    } else {
        None
    };

    let question = cx.props.question;

    let press = move |_| {
        to_owned!(client, question);

        cx.spawn({
            async move {
                if let Some(client) = client {
                    let client = client.lock().await;
                    client.send_string(
                        &serde_json::to_string(&ClientMessage::Input(InputEvent::SelectQuestion(
                            question,
                        )))
                        .unwrap(),
                    );
                }
            }
        });
    };

    if !cx.props.answered {
        cx.render(rsx! { div { onclick: press, class: "question", "{cx.props.cost}" } })
    } else {
        cx.render(rsx! { div { class: "question answered", "{cx.props.cost}" } })
    }
}
