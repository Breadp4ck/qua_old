use serde::{Deserialize, Serialize};
use std::fs;

use qua_game::package::prelude::*;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PackageConfig {
    pub info: Info,
    pub rounds: Vec<RoundData>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Info {
    pub name: String,
    pub version: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct RoundData {
    pub name: String,
    pub themes: Vec<ThemeData>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ThemeData {
    pub name: String,
    pub items: Vec<ItemData>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ItemData {
    pub cost: i32,
    pub title: String,
    pub answer: String,
    pub question_content: QuestionContent,
    pub question_description: Option<String>,
    pub answer_content: AnswerContent,
    pub answer_description: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(tag = "question_content_type")]
pub enum QuestionContent {
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
    Empty, // TODO: Remove
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(tag = "answer_content_type")]
pub enum AnswerContent {
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
    Empty, // TODO: Remove
}

impl Into<PackageState> for PackageConfig {
    fn into(self) -> PackageState {
        let mut game_package = PackageState::default();

        for round in &self.rounds {
            let mut game_round = RoundState { themes: vec![] };

            for theme in &round.themes {
                let mut game_theme = ThemeState { questions: vec![] };

                for item in &theme.items {
                    let game_question = QuestionState {
                        answered: false,
                        cost: item.cost.into(),
                        answered_by: None,
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

impl PackageConfig {
    pub fn from_toml(package_content: &str) -> Self {
        let package: Self =
            toml::from_str(&package_content).expect("Failure reading package");

        package
    }

    pub fn from_file(path: &str) -> Self {
        let package_content =
            fs::read_to_string(path).expect("Should have been able to read the file");

        Self::from_toml(&package_content)
    }
}
