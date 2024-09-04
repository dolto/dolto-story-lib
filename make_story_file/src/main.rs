use std::{
    collections::HashSet,
    io::{stdin, stdout, Read, Write},
};

fn main() {
    // todo : 다음과 같은 형식의 메세지를 rust 코드로 변환
    // input
    // 테스트 스크립트
    // 히오스를 좋아하는 두 친구가 대화한다.
    // 등장인물1 : 안녕?
    // 등장인물2 : 잘지내?
    // output
    // pub fn 테스트_스크립트() -> Vec<Story>{
    //    let mut base_story = Story::default();
    //    let mut 등장인물1_story = base_story.clone().title(TextPrint::parse("{{}}등장인물1".to_string()));
    //    let mut 등장인물2_story = base_story.clone().title(TextPrint::parse("{{}}등장인물2".to_string()));
    //    let 등장인물1_color = "input color";
    //    let 등장인물2_color = "input color";
    //    vec![
    //        base_story.clone().msg(TextPrint::parse(format!("{{{{}}}}히오스를 좋아하는 두 친구가 대화한다."))),
    //        등장인물1_story.clone().msg(TextPrint::parse(format!("{{{{color:{등장인물1_color}}}}}안녕?"))),
    //        등장인물2_story.clone().msg(TextRpint::parse(format!("{{{{color:{등장인물2_color}}}}}잘지내?"))),
    //    ]
    // }

    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split("\n");

    let title = input.next().unwrap().trim().replace(" ", "_");

    let mut characters = HashSet::new();

    let mut after_output = String::from("vec![\n");
    while let Some(script) = input.next() {
        if script == "" {
            continue;
        }
        let script: Vec<&str> = script.split(":").collect();
        if script.len() == 2 {
            let name = script[0].trim();
            let value = script[1].trim();
            characters.insert(name);
            after_output.push_str(
                format!(
                    "\t\t{name}_story.clone().msg(TextPrint::parse(format!(\"{{{{{{{{color:{{{name}_color}}}}}}}}}}{value}\"))),\n"
                )
                .as_str(),
            );
        } else {
            let value = script[0];
            after_output.push_str(
                format!("\t\tbase_story.clone().msg(TextPrint::parse(format!(\"{{{{{{{{}}}}}}}}{value}\"))),\n").as_str(),
            );
        }
    }
    after_output.push_str("\n\t]\n}");

    let mut characters: Vec<&str> = characters.into_iter().collect();
    characters.sort();
    let mut before_output =
        format!("pub fn {title}() -> Vec<Story>{{\n\tlet mut base_story = Story::default();\n");
    characters.into_iter().for_each(|ch| {
        before_output.push_str(
            format!("\tlet mut {ch}_story = base_story.clone().title(TextPrint::parse(format!(\"{{{{{{{{}}}}}}}}{ch}\")));\n\tlet mut {ch}_color = \"input_color\";\n").as_str()
        );
    });

    stdout()
        .write_all(format!("{before_output}{after_output}").as_bytes())
        .unwrap();
}
