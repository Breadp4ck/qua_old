use serde::{Deserialize, Serialize};
use std::fs;

use qua_game::package::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct PackageResource {
    info: Info,
    rounds: Vec<RoundData>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Info {
    name: String,
    version: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct RoundData {
    name: String,
    themes: Vec<ThemeData>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ThemeData {
    name: String,
    items: Vec<ItemData>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ItemData {
    cost: i32,
    title: String,
    answer: String,
    question_content: QuestionContent,
    question_description: String,
    answer_content: AnswerContent,
    answer_description: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "question_content_type")]
enum QuestionContent {
    Text {
        text_src: String,
    },
    Picture {
        picture_src: String,
    },
    Sound {
        sound_src: String,
        cover_src: Option<String>,
    },
    Video {
        video_src: String,
    },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "answer_content_type")]
enum AnswerContent {
    Text {
        text_src: String,
    },
    Picture {
        picture_src: String,
    },
    Sound {
        sound_src: String,
        cover_src: Option<String>,
    },
    Video {
        video_src: String,
    },
}

impl Into<Package> for PackageResource {
    fn into(self) -> Package {
        let mut game_package = Package::default();

        for round in &self.rounds {
            let mut game_round = Round { themes: vec![] };

            for theme in &round.themes {
                let mut game_theme = Theme { questions: vec![] };

                for item in &theme.items {
                    let game_question = Question {
                        answered: false,
                        cost: item.cost.into(),
                        answered_by: Vec::new(),
                    };

                    game_theme.questions.push(game_question);
                }

                game_round.themes.push(game_theme);
            }

            game_package.rounds.push(game_round);
        }

        game_package
    }
}

impl PackageResource {
    pub fn new(package_content: &str) -> Self {
        let package: PackageResource =
            toml::from_str(&package_content).expect("Failure reading package");

        package
    }

    pub fn from_file(path: &str) -> Self {
        let package_content =
            fs::read_to_string(path).expect("Should have been able to read the file");

        Self::new(&package_content)
    }
}
