use std::{
    fmt::{Debug, Display},
    rc::Rc,
};

use dioxus::prelude::*;
use gloo_timers::future::TimeoutFuture;
use tracing::info;
// use web_sys::{AudioContext, AudioContextState};

use crate::sound_effect::SoundEffect;

pub static GAMESTATE: GlobalSignal<Element> = Signal::global(|| None);
pub static LOG: GlobalSignal<Vec<Vec<TextPrint>>> = Signal::global(|| vec![]);

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
            size: 1.,
            speed: 20,
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
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ImagePrint {
    pub style: String,
    pub name: String,
    pub class: String,
}
impl ImagePrint {
    pub fn new(name: &str, style: &str, class: &str) -> Self {
        let name = name.to_owned();
        let style = style.to_owned();
        let class = class.to_owned();
        ImagePrint { name, style, class }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Story {
    pub msg: Vec<TextPrint>,
    pub left_img: Vec<ImagePrint>,
    pub right_img: Vec<ImagePrint>,
    pub background: String,
    pub class: String,
}
impl Story {
    pub fn new(
        msg: Vec<TextPrint>,
        left_img: Vec<ImagePrint>,
        right_img: Vec<ImagePrint>,
        background: &str,
        class: &str,
    ) -> Self {
        let background = background.to_owned();
        let class = class.to_owned();
        Story {
            msg,
            left_img,
            right_img,
            background,
            class,
        }
    }
}

#[component]
pub fn StoryPage(storys: Vec<Story>, next: Element) -> Element {
    let mut story_index = use_signal(|| 0_usize);
    let story = use_memo(move || storys.get(story_index()).cloned());

    let background = if let Some(s) = story() {
        s.background
    } else {
        "".to_string()
    };
    let class = if let Some(s) = story() {
        s.class
    } else {
        "".to_string()
    };

    if story().is_none() {
        *GAMESTATE.write() = next.clone();
    }

    rsx! {
        main{
            style: "{background}",
            class: "{class}",
            section{
                class: "relative x-screen y-screen",
                article{
                    class: "image",
                    if let Some(s) = &story(){
                        for (index,image) in s.left_img.iter().enumerate() {
                            img{
                                loading: "eager",
                                key: "{image.name}{image.style}",
                                class: "bottom-ground {image.class}",
                                style: "{image.style}left: {index as i32*5-5}rem;z-index:{index};",
                                src: "{image.name}"
                            }
                        }
                    }
                }
                article{
                    class: "image",
                    if let Some(s) = &story(){
                        for (index,image) in s.right_img.iter().enumerate() {
                            img{
                                loading: "eager",
                                key: "{image.name}{image.style}",
                                class: "bottom-ground {image.class}",
                                style: "{image.style}right: {index as i32*5-5}rem;z-index:{index};transform: scaleX(-1);",
                                src: "{image.name}"
                            }
                        }
                    }
                }
            }
            StoryBox{
                box_class: "fixed f-middle bottom-ground x-pd msg-box",
                box_style: "",
                can_skip: true,
                story: story().map_or_else(|| vec![], |s| s.msg),
                show_log: true,
                on_next: move |_|{
                    *story_index.write() += 1;
                    info!("{}", story_index());
                }
            }
        }
    }
}

struct DummyData {}
#[component]
fn StoryBox(
    story: Vec<TextPrint>,
    can_skip: bool,
    box_style: String,
    box_class: String,
    on_next: EventHandler<DummyData>,
    show_log: bool,
) -> Element {
    let mut end = use_signal(|| false);
    let mut skip = use_signal(|| false);
    let mut log = use_signal(|| false);
    let mut text_index = use_signal(|| 0_usize);
    let text_print = use_memo(use_reactive((&story,), |(story,)| story));
    let mut msg_index = use_signal(|| 0_usize);
    let message_len = use_memo(move || {
        if let Some(msg) = text_print().get(text_index()) {
            msg.msg.chars().count()
        } else {
            0
        }
    });

    let before_message = use_memo(move || {
        text_print()
            .iter()
            .take(text_index())
            .map(|txt| txt.print())
            .collect::<Vec<Element>>()
    });
    let message = use_memo(move || {
        if let Some(msg) = text_print().get(text_index()) {
            msg.part_print(msg_index())
        } else {
            None
        }
    });

    use_future(move || async move {
        loop {
            if let Some(msg) = text_print().get(text_index()) {
                *end.write() = false;
                if skip() {
                    TimeoutFuture::new(5).await;
                } else {
                    TimeoutFuture::new(msg.speed).await;
                }
                if !log() {
                    if message_len() > msg_index() {
                        *msg_index.write() += 1;
                        if let Some(s) = &msg.sound {
                            s().play().unwrap();
                        }
                    } else {
                        *msg_index.write() = 0;
                        *text_index.write() += 1;
                    }
                }
            } else {
                TimeoutFuture::new(10).await;
                if skip() {
                    LOG.write().push(text_print().clone());
                    on_next.call(DummyData {});
                    *text_index.write() = 0;
                    *msg_index.write() = 0;
                }
                *end.write() = true;
            }
        }
    });
    let keydown = move |e: KeyboardEvent| {
        if (e.code() == Code::ControlLeft || e.code() == Code::ControlRight) && can_skip {
            skip.set(true);
            info!("{}", skip());
        }
    };
    let keyup = move |e: KeyboardEvent| {
        if (e.code() == Code::ControlLeft || e.code() == Code::ControlRight) && can_skip {
            skip.set(false);
        }
    };
    let click = move |_: MouseEvent| {
        if end() {
            LOG.write().push(text_print().clone());
            on_next.call(DummyData {});
            *end.write() = false;
            *text_index.write() = 0;
            *msg_index.write() = 0;
        } else if can_skip {
            *text_index.write() = text_print().len();
        }
    };
    rsx! {
        if log() {
            section {
                class: "message-log",
                for l in LOG().iter(){
                    article{
                        for text in l.iter(){
                            {text.print()}
                        }
                    }
                }
                nav{
                    class: "msg-log-button",
                    onclick: move |e| {
                        *log.write() = false;
                        e.stop_propagation();
                    },
                    "exit",
                }
            }
        }else {
            article{
                class: "{box_class}",
                style: "{box_style}",
                onkeydown: keydown,
                onkeyup: keyup,
                onclick: click,
                tabindex: 1,
                autofocus: true,
                for t in before_message().iter(){
                    {t}
                }
                {message}
                nav{
                    class: "msg-log-button",
                    onclick: move |e|{
                        *log.write() = true;
                        e.stop_propagation();
                    },
                    "log",
                }
            }
        }
    }
}

#[component]
pub fn LightMessageBox(
    storys: Vec<Story>,
    box_style: String,
    can_skip: bool,
    box_class: String,
    show_log: bool,
) -> Element {
    let mut story_index = use_signal(|| 0_usize);
    rsx! {
        StoryBox{
            story: storys[story_index()].msg.clone(),
            box_style: box_style,
            can_skip: can_skip,
            box_class: box_class,
            on_next: move |_| {
                *story_index.write() += 1;
            },
            show_log: show_log,
        }

    }
}
