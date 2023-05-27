use qua_game::{game::Question, package::prelude::*, scores::Scores};
use qua_package::package_config::{
    AnswerContent, ItemData, PackageConfig, QuestionContent, RoundData, ThemeData,
};
use std::{
    collections::HashMap,
    io::{Cursor, Read},
    path::Path,
};
use web_sys::*;
use zip::{read::ZipFile, ZipArchive};

pub struct PackageResourceItem {
    pub title: String,
    pub cost: Scores,
    pub question_url_content: ResourceUrlContent,
    pub question_description: String,
    pub answer_url_content: ResourceUrlContent,
    pub answer_description: String,
}

#[derive(Clone)]
pub enum ResourceUrlContent {
    Text { url: String },
    Picture { url: String },
    Video { url: String },
    Sound { url: String },
    Empty,
}

pub struct PackageResource {
    config: PackageConfig,
    urls: HashMap<Question, (ResourceUrlContent, ResourceUrlContent)>,
}

impl PackageResource {
    pub fn new(binary_data: &[u8]) -> Self {
        let mut zip = zip::ZipArchive::new(Cursor::new(binary_data)).unwrap();

        let config = if let Ok(mut config) = zip.by_name("Pack.toml") {
            let mut config_string = String::new();
            config.read_to_string(&mut config_string).unwrap();

            PackageConfig::from_toml(&config_string) // TODO: result
        } else {
            panic!("Could not find Pack.toml") // TODO: error: Pack.toml not found
        };

        let mut urls: HashMap<Question, (ResourceUrlContent, ResourceUrlContent)> =
            HashMap::new();

        for (round_idx, round) in config.rounds.iter().enumerate() {
            for (theme_idx, theme) in round.themes.iter().enumerate() {
                for (item_idx, item) in theme.items.iter().enumerate() {
                    let question_url_content = match &item.question_content {
                        QuestionContent::Text { text_src } => {
                            let file = zip.by_name(&text_src).unwrap();
                            let url = Self::create_url(file);
                            ResourceUrlContent::Text { url }
                        }
                        QuestionContent::Picture { picture_src } => {
                            let file = zip.by_name(&picture_src).unwrap();
                            let url = Self::create_url(file);
                            ResourceUrlContent::Picture { url }
                        }
                        QuestionContent::Sound {
                            sound_src,
                            cover_src,
                        } => {
                            let file = zip.by_name(&sound_src).unwrap();
                            let url = Self::create_url(file);
                            ResourceUrlContent::Sound { url }
                        }
                        QuestionContent::Video { video_src } => {
                            let file = zip.by_name(&video_src).unwrap();
                            let url = Self::create_url(file);
                            ResourceUrlContent::Video { url }
                        }
                        QuestionContent::Empty => ResourceUrlContent::Empty,
                    };

                    let answer_url_content = match &item.answer_content {
                        AnswerContent::Text { text_src } => {
                            let file = zip.by_name(&text_src).unwrap();
                            let url = Self::create_url(file);
                            ResourceUrlContent::Text { url }
                        }
                        AnswerContent::Picture { picture_src } => {
                            let file = zip.by_name(&picture_src).unwrap();
                            let url = Self::create_url(file);
                            ResourceUrlContent::Picture { url }
                        }
                        AnswerContent::Sound {
                            sound_src,
                            cover_src,
                        } => {
                            let file = zip.by_name(&sound_src).unwrap();
                            let url = Self::create_url(file);
                            ResourceUrlContent::Sound { url }
                        }
                        AnswerContent::Video { video_src } => {
                            let file = zip.by_name(&video_src).unwrap();
                            let url = Self::create_url(file);
                            ResourceUrlContent::Video { url }
                        }
                        AnswerContent::Empty => ResourceUrlContent::Empty,
                    };

                    let index = Question::Normal(
                        round_idx.into(),
                        theme_idx.into(),
                        item_idx.into(),
                    );
                    urls.insert(index, (question_url_content, answer_url_content));
                }
            }
        }

        Self { config, urls }
    }

    fn create_url(mut file: ZipFile) -> String {
        let mut data = vec![];
        file.read_to_end(&mut data).unwrap();

        let extension = Path::new(file.name())
            .extension()
            .unwrap()
            .to_str()
            .unwrap();

        let mut properties = BlobPropertyBag::new();

        match extension {
            "webm" => {
                properties.type_("video/webm");
            }
            "mp4" => {
                properties.type_("video/mp4");
            }
            "png" => {
                properties.type_("video/png");
            }
            "jpg" | "jpeg" => {
                properties.type_("video/jpeg");
            }
            "txt" => {
                properties.type_("text/plain");
            }
            "txt" => {
                properties.type_("text/x-markdown");
            }
            "mp3" => {
                properties.type_("audio/mp3");
            }
            _ => panic!("Unsupported file type: {}", extension),
        }

        let uint8arr = unsafe { js_sys::Uint8Array::view(&data) };
        let array = js_sys::Array::new();
        array.push(&uint8arr);

        let blob = Blob::new_with_u8_array_sequence_and_options(&array, &properties).unwrap();
        Url::create_object_url_with_blob(&blob).unwrap()
    }

    fn validate(&mut self) {}

    pub fn get(&self, question: Question) -> PackageResourceItem {
        let (question_url_content, answer_url_content) = self.urls.get(&question).unwrap(); // TODO: wrap result
        let item_data = self.get_item_data(question).unwrap(); // TODO: wrap result

        PackageResourceItem {
            title: item_data.title,
            cost: item_data.cost.into(),
            question_url_content: question_url_content.clone(),
            question_description: item_data.question_description,
            answer_url_content: answer_url_content.clone(),
            answer_description: item_data.answer_description,
        }
    }

    fn get_item_data(&self, question: Question) -> Option<ItemData> {
        match question {
            Question::Normal(round_index, theme_index, question_index) => {
                let round = &self.config.rounds[round_index.idx()];
                let theme = &round.themes[theme_index.idx()];
                let question = &theme.items[question_index.idx()];

                return Some(question.clone());
            }
            Question::Final(question) => {
                todo!()
            }
        }

        None
    }
}
