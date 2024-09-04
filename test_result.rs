pub fn 테스트_스크립트() -> Vec<Story> {
    let mut base_story = Story::default();
    let mut 등장인물1_story = base_story
        .clone()
        .title(TextPrint::parse(format!("{{{{}}}}등장인물1")));
    let mut 등장인물1_color = "input_color";
    let mut 등장인물2_story = base_story
        .clone()
        .title(TextPrint::parse(format!("{{{{}}}}등장인물2")));
    let mut 등장인물2_color = "input_color";
    vec![
        등장인물1_story.clone().msg(TextPrint::parse(format!(
            "{{{{color:등장인물1_color}}}}안녕?"
        ))),
        등장인물2_story.clone().msg(TextPrint::parse(format!(
            "{{{{color:등장인물2_color}}}}반가워!"
        ))),
        base_story
            .clone()
            .msg(TextPrint::parse(format!("{{{{}}}}"))),
    ]
}
