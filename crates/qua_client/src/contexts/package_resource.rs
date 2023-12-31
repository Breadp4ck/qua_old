use qua_game::{
    game::Question,
    prelude::{Round, Theme},
    scores::Scores,
};
use qua_package::package_config::{AnswerContent, Info, ItemData, PackageConfig, QuestionContent};
use std::{
    collections::HashMap,
    io::{Cursor, Read, Write},
    path::Path,
};
use web_sys::*;
use zip::{read::ZipFile, ZipWriter};

use crate::components::prelude::{AnswersData, QuestionsData};

pub struct PackageResourceItem {
    pub title: String,
    pub answer: String,
    pub cost: Scores,
    pub question_url_content: ResourceContent,
    pub question_description: Option<String>,
    pub answer_url_content: ResourceContent,
    pub answer_description: Option<String>,
}

#[derive(Clone)]
pub enum ResourceContent {
    Text { text: String },
    Picture { url: String },
    Video { url: String },
    Sound { url: String },
    Empty,
}

pub struct PackageResource {
    config: PackageConfig,
    urls: HashMap<Question, (ResourceContent, ResourceContent)>,
}

impl PackageResource {
    pub fn new(binary_data: &[u8]) -> Result<Self, String> {
        let mut zip = zip::ZipArchive::new(Cursor::new(binary_data))
            .map_err(|_| "Could not create ZipArchive".to_string())?;

        let config = if let Ok(mut config) = zip.by_name("Pack.toml") {
            let mut config_string = String::new();
            config
                .read_to_string(&mut config_string)
                .map_err(|err| err.to_string())?;

            PackageConfig::from_toml(&config_string)
        } else {
            return Err("Could not find Pack.toml".into());
        };

        let mut urls: HashMap<Question, (ResourceContent, ResourceContent)> = HashMap::new();

        for (round_idx, round) in config.rounds.iter().enumerate() {
            for (theme_idx, theme) in round.themes.iter().enumerate() {
                for (item_idx, item) in theme.items.iter().enumerate() {
                    let question_url_content = match &item.question_content {
                        QuestionContent::Text { text } => {
                            ResourceContent::Text { text: text.clone() }
                        }
                        QuestionContent::Picture { picture_src } => {
                            let file = zip.by_name(&picture_src).unwrap();
                            let url = Self::create_url(file);
                            ResourceContent::Picture { url }
                        }
                        QuestionContent::Sound {
                            sound_src,
                            cover_src,
                        } => {
                            let file = zip.by_name(&sound_src).unwrap();
                            let url = Self::create_url(file);
                            ResourceContent::Sound { url }
                        }
                        QuestionContent::Video { video_src } => {
                            let file = zip.by_name(&video_src).unwrap();
                            let url = Self::create_url(file);
                            ResourceContent::Video { url }
                        }
                        QuestionContent::Empty => ResourceContent::Empty,
                    };

                    let answer_url_content = match &item.answer_content {
                        AnswerContent::Text { text } => {
                            ResourceContent::Text { text: text.clone() }
                        }
                        AnswerContent::Picture { picture_src } => {
                            let file = zip.by_name(&picture_src).unwrap();
                            let url = Self::create_url(file);
                            ResourceContent::Picture { url }
                        }
                        AnswerContent::Sound {
                            sound_src,
                            cover_src,
                        } => {
                            let file = zip.by_name(&sound_src).unwrap();
                            let url = Self::create_url(file);
                            ResourceContent::Sound { url }
                        }
                        AnswerContent::Video { video_src } => {
                            let file = zip.by_name(&video_src).unwrap();
                            let url = Self::create_url(file);
                            ResourceContent::Video { url }
                        }
                        AnswerContent::Empty => ResourceContent::Empty,
                    };

                    let index = Question::Normal(round_idx, theme_idx, item_idx);
                    urls.insert(index, (question_url_content, answer_url_content));
                }
            }
        }

        Ok(Self { config, urls })
    }

    pub fn new_as_parts(
        binary_data: &[u8],
    ) -> (Result<
        (
            PackageConfig,
            HashMap<Question, Vec<u8>>,
            HashMap<Question, Vec<u8>>,
        ),
        String,
    >) {
        let mut questions: HashMap<Question, Vec<u8>> = HashMap::new();
        let mut answers: HashMap<Question, Vec<u8>> = HashMap::new();
        let mut zip = zip::ZipArchive::new(Cursor::new(binary_data))
            .map_err(|_| "Could not create ZipArchive".to_string())?;

        let config = if let Ok(mut config) = zip.by_name("Pack.toml") {
            let mut config_string = String::new();
            config
                .read_to_string(&mut config_string)
                .map_err(|err| err.to_string())?;

            PackageConfig::from_toml(&config_string)
        } else {
            return Err("Could not find Pack.toml".into());
        };

        let mut urls: HashMap<Question, (ResourceContent, ResourceContent)> = HashMap::new();

        for (round_idx, round) in config.rounds.iter().enumerate() {
            for (theme_idx, theme) in round.themes.iter().enumerate() {
                for (question_idx, item) in theme.items.iter().enumerate() {
                    match &item.question_content {
                        QuestionContent::Text { text } => (),
                        QuestionContent::Picture { picture_src } => {
                            let mut file = zip.by_name(&picture_src).unwrap();
                            let mut data = vec![];
                            file.read_to_end(&mut data).unwrap();

                            questions
                                .insert(Question::Normal(round_idx, theme_idx, question_idx), data);
                        }
                        QuestionContent::Sound {
                            sound_src,
                            cover_src,
                        } => {
                            let mut file = zip.by_name(&sound_src).unwrap();
                            let mut data = vec![];
                            file.read_to_end(&mut data).unwrap();

                            questions
                                .insert(Question::Normal(round_idx, theme_idx, question_idx), data);
                        }
                        QuestionContent::Video { video_src } => {
                            let mut file = zip.by_name(&video_src).unwrap();
                            let mut data = vec![];
                            file.read_to_end(&mut data).unwrap();

                            questions
                                .insert(Question::Normal(round_idx, theme_idx, question_idx), data);
                        }
                        QuestionContent::Empty => (),
                    };

                    match &item.answer_content {
                        AnswerContent::Text { text } => (),
                        AnswerContent::Picture { picture_src } => {
                            let mut file = zip.by_name(&picture_src).unwrap();
                            let mut data = vec![];
                            file.read_to_end(&mut data).unwrap();

                            answers
                                .insert(Question::Normal(round_idx, theme_idx, question_idx), data);
                        }
                        AnswerContent::Sound {
                            sound_src,
                            cover_src,
                        } => {
                            let mut file = zip.by_name(&sound_src).unwrap();
                            let mut data = vec![];
                            file.read_to_end(&mut data).unwrap();

                            answers
                                .insert(Question::Normal(round_idx, theme_idx, question_idx), data);
                        }
                        AnswerContent::Video { video_src } => {
                            let mut file = zip.by_name(&video_src).unwrap();
                            let mut data = vec![];
                            file.read_to_end(&mut data).unwrap();

                            answers
                                .insert(Question::Normal(round_idx, theme_idx, question_idx), data);
                        }
                        AnswerContent::Empty => (),
                    };
                }
            }
        }

        Ok((config, questions, answers))
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
                properties.type_("image/png");
            }
            "jpg" | "jpeg" => {
                properties.type_("image/jpeg");
            }
            "svg" => {
                properties.type_("image/svg+xml");
            }
            "txt" => {
                properties.type_("text/plain");
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

    pub fn get_info(&self) -> Info {
        self.config.info.clone()
    }

    pub fn get_round(&self, round: Round) -> String {
        match round {
            Round::Normal(round_idx) => self.config.rounds[round_idx].name.clone(),
            Round::Final => {
                todo!()
            }
        }
    }

    pub fn get_theme(&self, theme: Theme) -> String {
        match theme {
            Theme::Normal(round_idx, theme_idx) => {
                self.config.rounds[round_idx].themes[theme_idx].name.clone()
            }
        }
    }

    pub fn get(&self, question: Question) -> PackageResourceItem {
        let (question_url_content, answer_url_content) = self.urls.get(&question).unwrap(); // TODO: wrap result
        let item_data = self.get_item_data(question).unwrap(); // TODO: wrap result

        PackageResourceItem {
            title: item_data.title,
            answer: item_data.answer,
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
                // TODO: Maybe return None
                let round = &self.config.rounds[round_index];
                let theme = &round.themes[theme_index];
                let question = &theme.items[question_index];

                return Some(question.clone());
            }
            Question::Final(question) => {
                todo!()
            }
        }
    }

    pub fn export(
        config: &PackageConfig,
        questions: &HashMap<Question, Vec<u8>>,
        answers: &HashMap<Question, Vec<u8>>,
    ) -> Vec<u8> {
        let mut output: Vec<u8> = Vec::new();

        {
            let mut zip = ZipWriter::new(Cursor::new(&mut output));
            let options = zip::write::FileOptions::default()
                .compression_method(zip::CompressionMethod::Stored);

            zip.add_directory("media/", options);

            for (round_idx, round) in config.rounds.iter().enumerate() {
                zip.add_directory(format!("media/{}", round_idx), options);

                for (theme_idx, theme) in round.themes.iter().enumerate() {
                    zip.add_directory(format!("media/{}/{}", round_idx, theme_idx), options);

                    for (question_idx, item) in theme.items.iter().enumerate() {
                        match &item.question_content {
                            QuestionContent::Text { text } => {}
                            QuestionContent::Picture { picture_src } => {
                                zip.start_file(&*picture_src, options);
                                zip.write(
                                    questions
                                        .get(&Question::Normal(round_idx, theme_idx, question_idx))
                                        .unwrap(),
                                )
                                .unwrap();
                            }
                            QuestionContent::Sound {
                                sound_src,
                                cover_src,
                            } => {
                                zip.start_file(&*sound_src, options);
                                zip.write(
                                    questions
                                        .get(&Question::Normal(round_idx, theme_idx, question_idx))
                                        .unwrap(),
                                )
                                .unwrap();
                            }
                            QuestionContent::Video { video_src } => {
                                zip.start_file(&*video_src, options);
                                zip.write(
                                    questions
                                        .get(&Question::Normal(round_idx, theme_idx, question_idx))
                                        .unwrap(),
                                )
                                .unwrap();
                            }
                            QuestionContent::Empty => (),
                        }

                        match &item.answer_content {
                            AnswerContent::Text { text } => {}
                            AnswerContent::Picture { picture_src } => {
                                zip.start_file(&*picture_src, options);
                                zip.write(
                                    answers
                                        .get(&Question::Normal(round_idx, theme_idx, question_idx))
                                        .unwrap(),
                                )
                                .unwrap();
                            }
                            AnswerContent::Sound {
                                sound_src,
                                cover_src,
                            } => {
                                zip.start_file(&*sound_src, options);
                                zip.write(
                                    answers
                                        .get(&Question::Normal(round_idx, theme_idx, question_idx))
                                        .unwrap(),
                                )
                                .unwrap();
                            }
                            AnswerContent::Video { video_src } => {
                                zip.start_file(&*video_src, options);
                                zip.write(
                                    answers
                                        .get(&Question::Normal(round_idx, theme_idx, question_idx))
                                        .unwrap(),
                                )
                                .unwrap();
                            }
                            AnswerContent::Empty => (),
                        }
                    }
                }
            }

            let config_string = toml::to_string(config).unwrap();
            zip.start_file("Pack.toml", options);
            zip.write(config_string.as_bytes()).unwrap();

            zip.finish().unwrap();
        }

        output
    }
}
