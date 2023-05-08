pub mod package_resource;

#[cfg(test)]
mod tests {
    use super::package_resource::PackageResource;

    #[test]
    fn empty_package() {
        let package_content = "
[info]
name = \"Пакет\"
version = \"0.1.0\"

[[rounds]]
name = \"Аниме\"

[[rounds.themes]]
name = \"Персонажи\"

[[rounds.themes.items]]
cost = 100
title = \"Назовите персонажа\" # general question objective
answer = \"Наруто\" # general answer
answer_description = \"\" # (can be empty, helps understanding)
question_description = \"\" # (can be empty, helps understanding)

[rounds.themes.items.question_content]
question_content_type = \"Picture\" # Text, Picture, Sound, Video

text_src = \"\" # for Text
picture_src = \"\" # for Picure
video_src = \"\" # for Video
sound_src = \"\" # for Sound
cover_src = \"\" # for Sound (can be empty)

[rounds.themes.items.answer_content]
answer_content_type = \"Text\" # Empty, Text, Picture, Sound, Video

text_src = \"\" # for Text
picture_src = \"\" # for Picure
video_src = \"\" # for Video
sound_src = \"\" # for Sound
cover_src = \"\" # for Sound (can be empty)
";

        let _package = PackageResource::new(package_content);
    }
}
