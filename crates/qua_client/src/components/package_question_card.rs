use crate::*;
use dioxus::prelude::*;
use qua_game::prelude::*;
use qua_package::package_config::*;

#[derive(PartialEq, Props)]
pub struct PackageQuestionItemProps {
    question: Question,
}

pub fn package_question_card(cx: Scope<PackageQuestionItemProps>) -> Element {
    let config = use_shared_state::<PackageConfig>(cx).unwrap();
    let questions = use_shared_state::<QuestionsData>(cx).unwrap();
    let answers = use_shared_state::<AnswersData>(cx).unwrap();
    let question = cx.props.question;

    let (item, idx) = match question {
        Question::Final(question_idx) => todo!(),
        Question::Normal(round_idx, theme_idx, question_idx) => (
            config.read().rounds[round_idx].themes[theme_idx].items[question_idx].clone(),
            question_idx,
        ),
    };

    let load_question_file = move |evt: Event<FormData>| {
        to_owned!(questions);

        async move {
            if let Some(file_engine) = &evt.files {
                let files = file_engine.files();
                for file_name in &files {
                    if let Some(file) = file_engine.read_file(file_name).await {
                        questions.write().insert(question, file);
                        log::info!("Loaded: {:?}", file_name);
                    }
                }
            }
        }
    };

    let load_answer_file = move |evt: Event<FormData>| {
        to_owned!(answers);

        async move {
            if let Some(file_engine) = &evt.files {
                let files = file_engine.files();
                for file_name in &files {
                    if let Some(file) = file_engine.read_file(file_name).await {
                        answers.write().insert(question, file);
                        log::info!("Loaded: {:?}", file_name);
                    }
                }
            }
        }
    };

    let change_question_content = move |question_type: &str| {
        to_owned!(config, question_type);

        cx.spawn({
            async move {
                let mut config = config.write();
                match question {
                    Question::Final(question_idx) => todo!(),
                    Question::Normal(round_idx, theme_idx, question_idx) => {
                        match question_type.as_str() {
                            "text" => {
                                config.rounds[round_idx].themes[theme_idx].items[question_idx]
                                    .question_content = QuestionContent::Text {
                                    text_src: "".into(),
                                }
                            }
                            "picture" => {
                                config.rounds[round_idx].themes[theme_idx].items[question_idx]
                                    .question_content = QuestionContent::Picture {
                                    picture_src: "".into(),
                                }
                            }
                            "sound" => {
                                config.rounds[round_idx].themes[theme_idx].items[question_idx]
                                    .question_content = QuestionContent::Sound {
                                    sound_src: "".into(),
                                    cover_src: None,
                                }
                            }
                            "video" => {
                                config.rounds[round_idx].themes[theme_idx].items[question_idx]
                                    .question_content = QuestionContent::Video {
                                    video_src: "".into(),
                                }
                            }
                            "empty" => {
                                config.rounds[round_idx].themes[theme_idx].items[question_idx]
                                    .question_content = QuestionContent::Empty
                            }
                            _ => (),
                        }
                    }
                }
            }
        });
    };

    let change_answer_content = move |answer_type: &str| {
        to_owned!(config, answer_type);

        cx.spawn({
            async move {
                let mut config = config.write();
                match question {
                    Question::Final(question_idx) => todo!(),
                    Question::Normal(round_idx, theme_idx, question_idx) => {
                        match answer_type.as_str() {
                            "text" => {
                                config.rounds[round_idx].themes[theme_idx].items[question_idx]
                                    .answer_content = AnswerContent::Text {
                                    text_src: "".into(),
                                }
                            }
                            "picture" => {
                                config.rounds[round_idx].themes[theme_idx].items[question_idx]
                                    .answer_content = AnswerContent::Picture {
                                    picture_src: "".into(),
                                }
                            }
                            "sound" => {
                                config.rounds[round_idx].themes[theme_idx].items[question_idx]
                                    .answer_content = AnswerContent::Sound {
                                    sound_src: "".into(),
                                    cover_src: None,
                                }
                            }
                            "video" => {
                                config.rounds[round_idx].themes[theme_idx].items[question_idx]
                                    .answer_content = AnswerContent::Video {
                                    video_src: "".into(),
                                }
                            }
                            "empty" => {
                                config.rounds[round_idx].themes[theme_idx].items[question_idx]
                                    .answer_content = AnswerContent::Empty
                            }
                            _ => (),
                        }
                    }
                }
            }
        });
    };

    let change_question_title = move |title: String| {
        to_owned!(config);

        cx.spawn({
            async move {
                let mut config = config.write();
                match question {
                    Question::Final(question_idx) => todo!(),
                    Question::Normal(round_idx, theme_idx, question_idx) => {
                        config.rounds[round_idx].themes[theme_idx].items[question_idx].title =
                            title;
                    }
                }
            }
        });
    };

    let change_question_answer = move |answer: String| {
        to_owned!(config);

        cx.spawn({
            async move {
                let mut config = config.write();
                match question {
                    Question::Final(question_idx) => todo!(),
                    Question::Normal(round_idx, theme_idx, question_idx) => {
                        config.rounds[round_idx].themes[theme_idx].items[question_idx].answer =
                            answer;
                    }
                }
            }
        });
    };

    let change_question_cost = move |cost: String| {
        to_owned!(config);

        cx.spawn({
            async move {
                let cost: i32 = cost.trim().parse().expect("Wanted a number");

                let mut config = config.write();
                match question {
                    Question::Final(question_idx) => todo!(),
                    Question::Normal(round_idx, theme_idx, question_idx) => {
                        config.rounds[round_idx].themes[theme_idx].items[question_idx].cost = cost;
                    }
                }
            }
        });
    };

    let remove_question = move |_| {
        to_owned!(config);

        cx.spawn({
            async move {
                let mut config = config.write();
                match question {
                    Question::Final(question_idx) => todo!(),
                    Question::Normal(round_idx, theme_idx, question_idx) => {
                        config.rounds[round_idx].themes[theme_idx]
                            .items
                            .remove(question_idx);
                    }
                }
            }
        });
    };

    cx.render(rsx! {
        form { class: "package-card",
            div { class: "header",
                div {
                    class: "text-edit",
                    "Question #{idx+1}"
                }
                div { class: "remove neutral-btn",
                    onclick: remove_question,
                    "x"
                }
            }
            div {
                class: "body",
                div {
                    class: "row",
                    div {
                        "title:"
                    }
                    input {
                        onchange: move |event| {
                            change_question_title(event.data.value.clone())
                        },
                        class: "text-edit",
                        r#type: "text",
                        placeholder: "Write what is required as an answer",
                        value: "{item.title}",
                    }
                }
                div {
                    class: "row",
                    div {
                        "cost:"
                    }
                    input {
                        onchange: move |event| {
                            change_question_cost(event.data.value.clone())
                        },
                        class: "text-edit",
                        r#type: "number",
                        min: 0,
                        step: 1,
                        placeholder: "Write question cost",
                        value: "{item.cost}",
                    }
                }
                div {
                    class: "row",
                    div {
                        "answer:"
                    }
                    input {
                        onchange: move |event| {
                            change_question_answer(event.data.value.clone())
                        },
                        class: "text-edit",
                        r#type: "text",
                        placeholder: "Write short answer",
                        value: "{item.answer}",
                    }
                }
                div {
                    class: "row",
                    div {
                        "full question:"
                    }
                    select {
                        class: "text-edit",
                        onchange: move |event| {
                            change_question_content(&event.value)
                        },
                        option {
                            value: "empty",
                            "--"
                        }
                        option {
                            value: "text",
                            "Text"
                        }
                        option {
                            value: "picture",
                            "Picture"
                        }
                        option {
                            value: "sound",
                            "Sound"
                        }
                        option {
                            value: "video",
                            "Video"
                        }
                    }
                    match item.question_content {
                        QuestionContent::Text { text_src } => rsx!{
                            input {
                                onchange: load_question_file,
                                r#type: "file",
                            }
                        },
                        QuestionContent::Picture { picture_src } => rsx! {
                            input {
                                onchange: load_question_file,
                                r#type: "file",
                            }
                        },
                        QuestionContent::Sound { sound_src, cover_src } => rsx! {
                            input {
                                onchange: load_question_file,
                                r#type: "file",
                            }
                        },
                        QuestionContent::Video { video_src } => rsx! {
                            input {
                                onchange: load_question_file,
                                r#type: "file",
                            }
                        },
                        QuestionContent::Empty => rsx! { div {} },
                    }
                }
                div {
                    class: "row",
                    div {
                        "full answer:"
                    }
                    select {
                        class: "text-edit",
                        onchange: move |event| {
                            change_answer_content(&event.value)
                        },
                        option {
                            value: "empty",
                            "--"
                        }
                        option {
                            value: "text",
                            "Text"
                        }
                        option {
                            value: "picture",
                            "Picture"
                        }
                        option {
                            value: "sound",
                            "Sound"
                        }
                        option {
                            value: "video",
                            "Video"
                        }
                    }
                    match item.answer_content {
                        AnswerContent::Text { text_src } => rsx!{
                            input {
                                onchange: load_answer_file,
                                r#type: "file",
                            }
                        },
                        AnswerContent::Picture { picture_src } => rsx! {
                            input {
                                onchange: load_answer_file,
                                r#type: "file",
                            }
                        },
                        AnswerContent::Sound { sound_src, cover_src } => rsx! {
                            input {
                                onchange: load_answer_file,
                                r#type: "file",
                            }
                        },
                        AnswerContent::Video { video_src } => rsx! {
                            input {
                                onchange: load_answer_file,
                                r#type: "file",
                            }
                        },
                        AnswerContent::Empty => rsx! { div {} },
                    }
                }
            }
        }
    })
}
