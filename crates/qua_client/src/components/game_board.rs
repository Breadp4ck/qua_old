use dioxus::prelude::*;
use fermi::{use_read, use_set};
use qua_game::prelude::*;

use super::prelude::*;
use crate::*;

pub fn game_board(cx: Scope) -> Element {
    let game = use_shared_state::<Game>(cx).unwrap().read().clone();
    let board = use_shared_state::<BoardUpdate>(cx).unwrap().read();
    let package = use_read(cx, PACKAGE_RESOURCE);

    let board = match &*board {
        BoardUpdate::Init => rsx! { div { class: "message", "Waiting for players" } },
        BoardUpdate::Greet => rsx! { div { class: "message", "qua!" } },
        BoardUpdate::Overview => rsx! { div { class: "message", "PACKAGE_THEMES_PLACEHOLDER" } },
        BoardUpdate::QuestionType(question) => rsx! { div { class: "message", "Внимание, вопрос!" } },
        BoardUpdate::QuestionMatter(question) => {
            rsx! { game_question_matter { question: question.clone() } }
        }
        BoardUpdate::QuestionMedia(question) => {
            rsx! {game_media_content { question: question.clone(), media_source: MediaSource::Question }}
        }
        BoardUpdate::AnswerMedia(question) => {
            rsx! {game_media_content { question: question.clone(), media_source: MediaSource::Answer }}
        }
        BoardUpdate::Picking(round) => {
            let round_idx = match round {
                Round::Normal(round) => round,
                Round::Final => todo!(),
            };

            let rounds = &game.package().rounds;
            if let Some(round) = rounds.get(*round_idx) {
                rsx! {
                    div { class: "round",
                        for (theme_idx , theme) in round.themes.iter().enumerate() {
                            div { class: "theme",
                                game_board_theme { theme: Theme::Normal(round_idx.clone(), theme_idx.into()) }
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
        div { class: "board", board }
    })
}
