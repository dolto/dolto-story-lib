use std::{
    collections::VecDeque,
    fmt::{Debug, Display},
    rc::Rc,
};

use dioxus::prelude::*;
use rand::{thread_rng, Rng};
// use tracing::info;
// use web_sys::{AudioContext, AudioContextState};

#[cfg(target_arch = "wasm32")]
pub async fn wait(mili: u32) {
    use gloo_timers::future::TimeoutFuture;
    TimeoutFuture::new(mili).await;
}
#[cfg(not(target_arch = "wasm32"))]
pub async fn wait(mili: u32) {
    use std::time::Duration;
    tokio::time::sleep(Duration::from_millis(mili as u64)).await;
}

use crate::sound_effect::{SoundEffect, SOUND_EFFECTS};

pub static GAMESTATE: GlobalSignal<Element> = Signal::global(|| None);
pub static LOG: GlobalSignal<Vec<Vec<TextPrint>>> = Signal::global(|| vec![]);
pub static TEXTCONFIG: GlobalSignal<TextConfig> = Signal::global(|| TextConfig {
    sound_volum: 1.,
    music_volum: 1.,
    speed: 1.,
    auto_speed: 5000,
    is_auto: false,
    is_skip: false,
    is_close: false,
});

#[derive(Debug, PartialEq, Clone)]
pub struct TextConfig {
    pub sound_volum: f64,
    pub music_volum: f64,
    pub speed: f32,
    pub auto_speed: u32,
    pub is_auto: bool,
    pub is_skip: bool,
    pub is_close: bool,
}
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

    pub fn parse(s: String) -> Vec<TextPrint> {
        let mut s: VecDeque<&str> = s.split("{{").skip(1).collect();
        let capacity = s.len();
        let mut result = Vec::with_capacity(capacity);

        while let Some(s) = s.pop_front() {
            let mut s = s.split("}}");
            let mut option = s.next().unwrap().split("|");
            let message = s.next().unwrap();

            let mut temp = TextPrint::default();
            temp = temp.msg(message);

            while let Some(op) = option.next() {
                let mut c_v = op.split(":");
                let command = c_v.next().unwrap().trim();
                let value = c_v.next().unwrap().trim();

                match command {
                    "color" => {
                        temp = temp.color(value);
                    }
                    "font" => {
                        temp = temp.font(value);
                    }
                    "font_weight" => {
                        temp = temp.font_weight(match value {
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
                        temp = temp.option(match value {
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
                        temp = temp.speed(value.parse::<u32>().unwrap_or(10));
                    }
                    "size" => {
                        temp = temp.size(value.parse::<f32>().unwrap_or(1.));
                    }
                    "style" => {
                        let mut v = value.split("(");
                        let command = v.next().unwrap();
                        let value = v.next().unwrap().split(")").next().unwrap().trim();
                        temp = temp.style(match command {
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
                                temp = temp.sound(Rc::clone(&animal_crossing));
                            }
                            _ => {}
                        }
                    }
                    "class" => {
                        temp = temp.class(value);
                    }
                    "is_split" => {
                        let v = if value.trim() == "true" { true } else { false };
                        temp = temp.is_split(v);
                    }
                    _ => {}
                }
            }
            result.push(temp);
        }
        result
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
    pub title: Vec<TextPrint>,
    pub msg: Vec<TextPrint>,
    pub center_img: Option<ImagePrint>,
    pub left_img: Vec<ImagePrint>,
    pub right_img: Vec<ImagePrint>,
    pub background: String,
    pub class: String,
}
impl Story {
    pub fn new(
        title: Vec<TextPrint>,
        msg: Vec<TextPrint>,
        left_img: Vec<ImagePrint>,
        center_img: Option<ImagePrint>,
        right_img: Vec<ImagePrint>,
        background: &str,
        class: &str,
    ) -> Self {
        let background = background.to_owned();
        let class = class.to_owned();
        Story {
            title,
            msg,
            center_img,
            left_img,
            right_img,
            background,
            class,
        }
    }

    pub fn title(mut self, title: Vec<TextPrint>) -> Self {
        self.title = title;
        self
    }

    pub fn msg(mut self, msg: Vec<TextPrint>) -> Self {
        self.msg = msg;
        self
    }
    pub fn add_left_img(mut self, img: ImagePrint) -> Self {
        self.left_img.push(img);
        self
    }
    pub fn change_left_img(mut self, img: ImagePrint, remove_index: usize) -> Self {
        self.left_img.push(img.clone());
        self.left_img.swap_remove(remove_index);
        self
    }
    pub fn add_right_img(mut self, img: ImagePrint) -> Self {
        self.right_img.push(img);
        self
    }
    pub fn change_right_img(mut self, img: ImagePrint, remove_index: usize) -> Self {
        self.right_img.push(img.clone());
        self.right_img.swap_remove(remove_index);
        self
    }

    pub fn background(mut self, background: &str) -> Self {
        self.background = background.to_owned();
        self
    }
    pub fn class(mut self, class: &str) -> Self {
        self.class = class.to_owned();
        self
    }
    pub fn change_center_img(mut self, img: ImagePrint) -> Self {
        self.center_img = Some(img);
        self
    }
    pub fn remove_center_img(mut self) -> Self {
        self.center_img = None;
        self
    }
}

#[component]
pub fn StoryPage(
    storys: Vec<Story>,
    next: Element,
    on_next: EventHandler<DummyData>,
    skip_len: usize,
) -> Element {
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
        on_next.call(DummyData {}); // 여기에 skip_len을 수정하는 로직을 만듦
        *GAMESTATE.write() = next.clone();
    }

    rsx! {
        main{
            style: "{background}",
            class: "{class}",
            onclick: move |_| {
                TEXTCONFIG.write().is_close = false;
            },
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
                    class: "imgae",
                    if let Some(s) = &story(){
                        if let Some(img) = s.center_img.clone(){
                            img{
                                loading: "eager",
                                class: "bottom-ground {img.class}",
                                style: "{img.style}left: 50%;transform: translateX(-50%);",
                                src: "{img.name}"
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
                skip_len: skip_len,
                title: story().map_or_else(|| vec![], |s| s.title),
                box_class: "fixed f-middle bottom-ground x-pd msg-box",
                box_style: "",
                can_skip: true,
                story: story().map_or_else(|| vec![], |s| s.msg),
                show_log: true,
                on_next: move |_|{
                    *story_index.write() += 1;
                }
            }
        }
    }
}

pub struct DummyData {}
#[component]
fn StoryBox(
    title: Vec<TextPrint>,
    story: Vec<TextPrint>,
    can_skip: bool,
    box_style: String,
    box_class: String,
    on_next: EventHandler<DummyData>,
    show_log: bool,
    skip_len: usize,
) -> Element {
    let mut text_config = use_signal(|| false);
    let _text_config = use_context_provider(move || text_config);
    let mut end = use_signal(|| false);
    let skip = use_memo(move || TEXTCONFIG().is_skip);
    let auto = use_memo(move || TEXTCONFIG().is_auto);
    let close = use_memo(move || TEXTCONFIG().is_close);
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
    let box_style = if close() {
        box_style + "visibility: collapse;"
    } else {
        box_style
    };

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
    let title: Vec<Option<VNode>> = title.iter().map(|t| t.print()).collect();

    use_future(move || async move {
        loop {
            if text_config() {
                wait(5).await;
            } else if let Some(msg) = text_print().get(text_index()) {
                *end.write() = false;
                if skip() {
                    wait(5).await;
                } else {
                    wait((msg.speed as f32 / TEXTCONFIG().speed) as u32).await;
                }
                if !log() {
                    if message_len() > msg_index() {
                        *msg_index.write() += 1;
                        if let Some(s) = &msg.sound {
                            if TEXTCONFIG().sound_volum != 0. {
                                s().play().unwrap();
                            }
                        }
                    } else {
                        *msg_index.write() = 0;
                        *text_index.write() += 1;
                    }
                }
            } else {
                wait(10).await;
                *end.write() = true;
                if skip() {
                    LOG.write().push(text_print().clone());
                    on_next.call(DummyData {});
                    *text_index.write() = 0;
                    *msg_index.write() = 0;
                } else if auto() {
                    let mut count = 0;
                    while count < TEXTCONFIG.read().auto_speed && end() && TEXTCONFIG.read().is_auto
                    {
                        wait(5).await;
                        count += 5;
                    }
                    if TEXTCONFIG.read().is_auto {
                        LOG.write().push(text_print().clone());
                        on_next.call(DummyData {});
                        *text_index.write() = 0;
                        *msg_index.write() = 0;
                    }
                }
            }
        }
    });
    let keydown = move |e: KeyboardEvent| {
        if (e.code() == Code::ControlLeft || e.code() == Code::ControlRight)
            && can_skip
            && skip_len > text_index()
        {
            TEXTCONFIG.write().is_skip = true;
        }
    };
    let keyup = move |e: KeyboardEvent| {
        if (e.code() == Code::ControlLeft || e.code() == Code::ControlRight) && can_skip {
            TEXTCONFIG.write().is_skip = false;
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
        if text_config(){
            Setting{}
        }else if log() {
            section {
                class: "message-log",
                for l in LOG().iter().rev(){
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
        } else {
            article{
                class: "{box_class}",
                style: "{box_style}",
                onkeydown: keydown,
                onkeyup: keyup,
                onclick: click,
                tabindex: 1,
                autofocus: true,
                nav{
                    class: "msg-log-button",
                    span{
                        onclick: move |e|{
                            *log.write() = true;
                            e.stop_propagation();
                        },
                        "log",
                    }
                    if can_skip{
                        span{
                            onclick: move |e|{
                                let auto = !TEXTCONFIG.read().is_auto;
                                TEXTCONFIG.write().is_auto = auto;
                                e.stop_propagation();
                            },
                            "auto"
                        }
                        span{
                            onclick: move |e|{
                                if skip_len > text_index(){
                                    let skip = !TEXTCONFIG.read().is_skip;
                                    TEXTCONFIG.write().is_skip = skip;
                                }
                                e.stop_propagation();
                            },
                            "skip"
                        }
                    }
                    span{
                        onclick: move |e|{
                            TEXTCONFIG.write().is_close = true;
                            e.stop_propagation();
                        },
                        "close"
                    }
                    span{
                        onclick: move |e|{
                            *text_config.write() = true;
                            e.stop_propagation();
                        },
                        "setting"
                    }
                }
                article{
                    if !title.is_empty(){
                        div{
                            class: "story-box-title",
                            for t in title.iter(){
                                {t}
                            }
                        }
                    }
                    for t in before_message().iter(){
                        {t}
                    }
                    {message}
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
    skip_len: usize,
) -> Element {
    let mut story_index = use_signal(|| 0_usize);
    rsx! {
        StoryBox{
            skip_len: skip_len,
            title: storys[story_index()].title.clone(),
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
#[component]
pub fn Setting() -> Element {
    let mut text_config = use_context::<Signal<bool>>();
    rsx! {
            section{
                class: "textconfig",
                label{"Music Volume: {TEXTCONFIG.read().music_volum}" }
                input{
                    r#type: "range",
                    min: 0.,
                    max: 2.,
                    step: 0.1,
                    value: "{TEXTCONFIG.read().music_volum}",
                    onchange: move |e|{
                        TEXTCONFIG.write().music_volum = e.data.value().parse().unwrap();
                        e.stop_propagation();
                    }
                }
                label{"Effect Volume: {TEXTCONFIG.read().sound_volum}" }
                input{
                    r#type: "range",
                    min: 0.,
                    max: 2.,
                    step: 0.1,
                    value: "{TEXTCONFIG.read().sound_volum}",
                    onchange: move |e|{
                        TEXTCONFIG.write().sound_volum = e.data.value().parse().unwrap();
                        e.stop_propagation();
                    }
                }
                label{"Base Speed: {TEXTCONFIG.read().speed}" }
                input{
                    r#type: "range",
                    min: 0.5,
                    max: 2.,
                    step: 0.1,
                    value: "{TEXTCONFIG.read().speed}",
                    onchange: move |e|{
                        TEXTCONFIG.write().speed = e.data.value().parse().unwrap();
                        e.stop_propagation();
                    }
                }
                label{"Auto Sleep: {TEXTCONFIG.read().auto_speed / 1000}.{TEXTCONFIG.read().auto_speed % 1000 / 100} sec" }
                input{
                    r#type: "range",
                    min: 0,
                    max: 20000,
                    step: 500,
                    value: "{TEXTCONFIG.read().auto_speed}",
                    onchange: move |e|{
                        TEXTCONFIG.write().auto_speed = e.data.value().parse().unwrap();
                        e.stop_propagation();
                    }
                }
                nav{
                    class: "setting-close",
                    onclick: move |e|{
                        *text_config.write() = false;
                        e.stop_propagation();
                    },
                    "exit"
                }
            }

    }
}
