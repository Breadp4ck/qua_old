use dioxus::prelude::*;
use qua_game::prelude::*;

use super::prelude::*;

pub fn game_board(cx: Scope) -> Element {
    let game = use_shared_state::<Game>(cx).unwrap().read().clone();
    let board = use_shared_state::<BoardUpdate>(cx).unwrap().read();

    let board = match &*board {
        BoardUpdate::Init => rsx! { div { class: "round", "ДАБРО ПАЖАЛОВАТЬ!!!" } },
        BoardUpdate::Text(text) => rsx! { div { "{text}" } },
        BoardUpdate::QuestionType(question) => rsx! { div { "ВОПРОСЕЦ" } },
        BoardUpdate::QuestionMatter(_) => rsx! { div {} },
        BoardUpdate::QuestionMedia(_) => rsx! { div {} },
        BoardUpdate::AnswerMedia(_) => rsx! { div {} },
        BoardUpdate::Picking(round) => {
            let round_idx = match round {
                qua_game::game::Round::Normal(round) => round,
                qua_game::game::Round::Final => todo!(),
            };

            let rounds = &game.package().rounds;
            if let Some(round) = rounds.get(round_idx.idx()) {
                rsx! {
                    div { class: "round",
                        for (theme_idx , theme) in round.themes.iter().enumerate() {
                            div { class: "theme",
                                div { class: "title", "Одна из тем" }
                                for (question_idx , question) in theme.questions.iter().enumerate() {
                                    game_board_question_button {
                                        cost: question.cost,
                                        answered: question.answered,
                                        question: Question::Normal(round_idx.clone(), theme_idx.into(), question_idx.into())
                                    }
                                }
                            }
                        }
                    }
                }
            } else {
                rsx! { div {} }
            }
        }
    };

    cx.render(rsx! {
        div { class: "game-board",
            div { class: "board", board }
            div { class: "timer", "" }
        }
    })
}
