use std::{
    collections::VecDeque,
    fmt::{Debug, Display},
    rc::Rc,
};

use dioxus::prelude::*;
use rand::{thread_rng, Rng};

use crate::sound_effect::{SoundEffect, SOUND_EFFECTS};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TextOption {
    Normal,
    Italic,
    Oblique(u32),
}
impl Display for TextOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TextOption::Normal => {
                write!(f, "normal")
            }
            TextOption::Italic => {
                write!(f, "italic")
            }
            TextOption::Oblique(deg) => {
                write!(f, "oblique {}deg", deg)
            }
        }
    }
}
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum FontWeight {
    Normal,
    Bold,
    Num(u32),
}
impl Display for FontWeight {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Normal => {
                write!(f, "normal")
            }
            Self::Bold => {
                write!(f, "bold")
            }
            Self::Num(size) => write!(f, "{}", size),
        }
    }
}
#[derive(Clone)]
pub struct TextPrint {
    pub color: String,
    pub option: TextOption,
    pub msg: String,
    pub size: f32,
    pub speed: u32,
    pub is_split: bool,
    pub style: Rc<dyn Fn() -> String>,
    pub font: String,
    pub font_weight: FontWeight,
    pub class: String,
    pub sound: Option<Rc<dyn Fn() -> SoundEffect>>,
}
impl PartialEq for TextPrint {
    fn eq(&self, other: &Self) -> bool {
        self.color == other.color
            && self.option == other.option
            && self.msg == other.msg
            && self.speed == other.speed
            && self.font == other.font
            && self.is_split == other.is_split
            && self.class == other.class
    }
}
impl Debug for TextPrint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "can't see debug mode")
    }
}
impl Default for TextPrint {
    fn default() -> Self {
        TextPrint {
            color: "black".to_owned(),
            option: TextOption::Normal,
            msg: "".to_owned(),
            size: 2.,
            speed: 60,
            is_split: false,
            style: Rc::new(|| "".to_owned()),
            font: "hancom-malang".to_owned(),
            font_weight: FontWeight::Normal,
            class: "".to_owned(),
            sound: None,
        }
    }
}
impl TextPrint {
    pub fn msg(mut self, msg: &str) -> Self {
        let msg = msg.to_owned();
        self.msg = msg;
        self
    }
    pub fn font(mut self, font: &str) -> Self {
        let font = font.to_owned();
        self.font = font;
        self
    }
    pub fn font_weight(mut self, font_weight: FontWeight) -> Self {
        self.font_weight = font_weight;
        self
    }
    pub fn option(mut self, option: TextOption) -> Self {
        self.option = option;
        self
    }
    pub fn speed(mut self, speed: u32) -> Self {
        self.speed = speed;
        self
    }
    pub fn color(mut self, color: &str) -> Self {
        self.color = color.to_owned();
        self
    }
    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }
    pub fn style(mut self, style: Rc<dyn Fn() -> String>) -> Self {
        self.style = style;
        self
    }
    pub fn sound(mut self, sound: Rc<dyn Fn() -> SoundEffect>) -> Self {
        self.sound = Some(sound);
        self
    }
    pub fn class(mut self, class: &str) -> Self {
        self.class = class.to_owned();
        self
    }
    pub fn is_split(mut self, is_split: bool) -> Self {
        self.is_split = is_split;
        self
    }
    pub fn color_bold(msg: &str, color: &str) -> Self {
        TextPrint::default()
            .msg(msg)
            .color(color)
            .speed(100)
            .font_weight(FontWeight::Bold)
    }
    pub fn new(
        msg: &str,
        font: &str,
        font_weight: FontWeight,
        option: TextOption,
        color: &str,
        size: f32,
        speed: u32,
        style: Rc<dyn Fn() -> String>,
        class: &str,
        is_split: bool,
    ) -> Self {
        let msg = msg.to_owned();
        let color = color.to_owned();
        let font = font.to_owned();
        let class = class.to_owned();
        TextPrint {
            color,
            option,
            msg,
            size,
            speed,
            is_split,
            style,
            font,
            font_weight,
            class,
            sound: None,
        }
    }
    fn global_print(text: &TextPrint) -> Element {
        let msgs = text.msg.split('\n').collect::<Vec<&str>>();
        let style = (text.style)();
        let out_class = if text.is_split {
            "".to_owned()
        } else {
            text.class.to_owned()
        };
        let out_style = if text.is_split {
            "".to_owned()
        } else {
            style.clone()
        };

        let print = |msg: String| {
            rsx! {
                if text.is_split{
                    for ch in msg.chars().collect::<Vec<char>>().iter() {
                        span{
                            class: "{text.class}",
                            style: "{(text.style)()}",
                            "{ch}"
                        }
                    }
                }else {
                    "{msg}"
                }
            }
        };
        rsx! {
            span{
                class: "{out_class}",
                style: r#"font-style: {text.option};font-size: {text.size}rem;font-family: "{text.font}";color: {text.color};font-weight: {text.font_weight};{out_style}"#,
                for (index,msg) in msgs.iter().enumerate() {
                    if index > 0{
                        {print(msg.to_string())}
                        br{}
                    }else {
                        {print(msg.to_string())}
                    }
                }
            }
        }
    }
    pub fn print(&self) -> Element {
        TextPrint::global_print(self)
    }
    pub fn part_print(&self, len: usize) -> Element {
        let msg = self.msg.chars().take(len).collect::<String>();
        let mut self_temp = self.clone();
        self_temp.msg = msg;
        TextPrint::global_print(&self_temp)
    }

    pub fn parse(s: String) -> Vec<TextPrint> {
        let mut textprint = TextPrint::default();
        let mut s: VecDeque<&str> = s.split("{{").skip(1).collect();
        let capacity = s.len();
        let mut result = Vec::with_capacity(capacity);

        while let Some(s) = s.pop_front() {
            let mut s = s.split("}}");
            let mut option = s.next().unwrap_or("").split("|");
            let message = s.next().unwrap();

            textprint = textprint.msg(message);

            while let Some(op) = option.next() {
                if op == "" {
                    continue;
                }
                let mut c_v = op.split(":");
                let command = c_v.next().unwrap().trim();
                let value = c_v.next().unwrap().trim();

                match command {
                    "default" => {
                        textprint = TextPrint::default();
                        textprint = textprint.msg(message);
                    }
                    "color" => {
                        textprint = textprint.color(value);
                    }
                    "font" => {
                        textprint = textprint.font(value);
                    }
                    "font_weight" => {
                        textprint = textprint.font_weight(match value {
                            "normal" => FontWeight::Normal,
                            "bold" => FontWeight::Bold,
                            v => {
                                let v = v
                                    .split("(")
                                    .skip(1)
                                    .next()
                                    .unwrap()
                                    .split(")")
                                    .next()
                                    .unwrap()
                                    .parse::<u32>()
                                    .unwrap_or(10);
                                FontWeight::Num(v)
                            }
                        })
                    }
                    "option" => {
                        textprint = textprint.option(match value {
                            "normal" => TextOption::Normal,
                            "italic" => TextOption::Italic,
                            v => {
                                let v = v
                                    .split("(")
                                    .skip(1)
                                    .next()
                                    .unwrap()
                                    .split(")")
                                    .next()
                                    .unwrap()
                                    .parse::<u32>()
                                    .unwrap_or(10);
                                TextOption::Oblique(v)
                            }
                        })
                    }
                    "speed" => {
                        textprint = textprint.speed(value.parse::<u32>().unwrap_or(10));
                    }
                    "size" => {
                        textprint = textprint.size(value.parse::<f32>().unwrap_or(1.));
                    }
                    "style" => {
                        let mut v = value.split("(");
                        let command = v.next().unwrap();
                        let value = v.next().unwrap().split(")").next().unwrap().trim();
                        textprint = textprint.style(match command {
                            "min_max4" => {
                                let mut min_max =
                                    value.split(",").flat_map(|i| i.trim().parse::<f32>());
                                let min = (min_max.next().unwrap(), min_max.next().unwrap());
                                let max = (min_max.next().unwrap(), min_max.next().unwrap());
                                let res: Rc<dyn Fn() -> String> = Rc::new(move || {
                                    let mut rng = thread_rng();
                                    let min = rng.gen_range(min.0..min.1);
                                    let max = rng.gen_range(max.0..max.1);
                                    format!("--min:{:.4}rem;--max:{:.4}rem", min, max)
                                });
                                res
                            }
                            "min_max2" => {
                                let mut min_max =
                                    value.split(",").flat_map(|i| i.trim().parse::<f32>());
                                let min = min_max.next().unwrap();
                                let max = min_max.next().unwrap();
                                let res: Rc<dyn Fn() -> String> = Rc::new(move || {
                                    format!("--min:{:.4}rem;--max:{:.4}rem", min, max)
                                });
                                res
                            }
                            _ => {
                                let res: Rc<dyn Fn() -> String> =
                                    Rc::new(|| format!("--min:1;--max:1"));
                                res
                            }
                        });
                    }
                    "sound" => {
                        let mut v = value.split("(");
                        let command = v.next().unwrap();
                        let value = v
                            .next()
                            .unwrap()
                            .split(")")
                            .next()
                            .unwrap()
                            .trim()
                            .to_string();
                        match command {
                            "animal_crossing" => {
                                let animal_crossing: Rc<dyn Fn() -> SoundEffect> =
                                    Rc::new(move || {
                                        let mut rng = thread_rng();
                                        SoundEffect::new(
                                            SOUND_EFFECTS().get(value.as_str()).unwrap().clone(),
                                        )
                                        .unwrap()
                                        .speed(rng.gen_range(1.0..3.))
                                        .volum(2.)
                                        .reverb(0.5)
                                        .is_rev(true)
                                    });
                                textprint = textprint.sound(Rc::clone(&animal_crossing));
                            }
                            _ => {}
                        }
                    }
                    "class" => {
                        textprint = textprint.class(value);
                    }
                    "is_split" => {
                        let v = if value.trim() == "true" { true } else { false };
                        textprint = textprint.is_split(v);
                    }
                    _ => {}
                }
            }
            result.push(textprint.clone());
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::TextPrint;

    #[test]
    fn text_print_parse() {
        let message =
            "{{}}메세지{{text_weight:bold|color:red}}메세지{{font_weight:normal}}메세지".to_owned();
        let text_parse = TextPrint::parse(message);
        let text_vec = vec![
            TextPrint::default().msg("메세지"),
            TextPrint::default()
                .msg("메세지")
                .font_weight(super::FontWeight::Bold)
                .color("red"),
            TextPrint::default().msg("메세지").color("red"),
        ];

        assert_eq!(text_parse[0], text_vec[0]);
        assert_eq!(text_parse[1], text_vec[1]);
        assert_eq!(text_parse[2], text_vec[2]);
        assert_eq!(text_parse, text_vec);
    }
}
