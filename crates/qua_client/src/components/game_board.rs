use dioxus::prelude::*;
use fermi::use_read;
use qua_game::prelude::*;

use crate::*;
use super::prelude::*;

pub fn game_board(cx: Scope) -> Element {
    let game = use_shared_state::<Game>(cx).unwrap().read().clone();
    let board = use_shared_state::<BoardUpdate>(cx).unwrap().read();

    let board = match &*board {
        BoardUpdate::Init => rsx! { div { class: "message", "ДАБРО ПАЖАЛОВАТЬ!!!" } },
        BoardUpdate::Text(text) => rsx! { div { class: "message", "{text}" } },
        BoardUpdate::QuestionType(question) => rsx! { div { class: "message", "ВОПРОСЕЦ" } },
        BoardUpdate::QuestionMatter(question) => {
            let question_idx = match question {
                Question::Final(_) => todo!(),
                Question::Normal(_, _, _) => {

                },
            };

            rsx! { div { class: "message", "ТЕМА ВОПРОСА" } }
        }
        BoardUpdate::QuestionMedia(question) => {
            rsx! {
                game_media_content {
                    question: question.clone(),
                    media_source: MediaSource::Question
                }
            }
        }
        BoardUpdate::AnswerMedia(question) => {
            rsx! {
                game_media_content {
                    question: question.clone(),
                    media_source: MediaSource::Answer
                }
            }
        }
        BoardUpdate::Picking(round) => {
            let round_idx = match round {
                Round::Normal(round) => round,
                Round::Final => todo!(),
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
