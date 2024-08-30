use std::fmt::Debug;

use crate::text_print::*;
use dioxus::prelude::*;
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
    is_setting: false,
    is_log: false,
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
    pub is_setting: bool,
    pub is_log: bool,
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
    pub fn remove_left_img(mut self, index: usize) -> Self {
        self.left_img.remove(index);
        self
    }
    pub fn pop_left_img(mut self) -> Self {
        self.left_img.pop();
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
    pub fn remove_right_img(mut self, index: usize) -> Self {
        self.right_img.remove(index);
        self
    }
    pub fn pop_right_img(mut self) -> Self {
        self.right_img.pop();
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
    skip: usize,
) -> Element {
    let mut story_index = use_signal(|| skip);
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
                    on_next.call(DummyData {}); // 여기에 skip_len을 수정하는 로직을 만듦
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
    let mut end = use_signal(|| false);
    let skip = use_memo(move || TEXTCONFIG().is_skip);
    let auto = use_memo(move || TEXTCONFIG().is_auto);
    let close = use_memo(move || TEXTCONFIG().is_close);
    let log = use_memo(move || TEXTCONFIG().is_log);
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
    let auto_clicked = if auto() { "is_clicked" } else { "" };
    let skip_clicked = if skip() { "is_clicked" } else { "" };

    use_future(move || async move {
        loop {
            if TEXTCONFIG().is_setting {
                wait(5).await;
            } else if let Some(msg) = text_print().get(text_index()) {
                *end.write() = false;
                if skip() && can_skip && skip_len > text_index() {
                    wait(5).await;
                } else if skip() {
                    TEXTCONFIG.write().is_skip = false;
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
                if skip() && can_skip && skip_len > text_index() {
                    LOG.write().push(text_print().clone());
                    on_next.call(DummyData {});
                    *text_index.write() = 0;
                    *msg_index.write() = 0;
                } else if skip() {
                    TEXTCONFIG.write().is_skip = false;
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
        if TEXTCONFIG().is_setting{
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
                        TEXTCONFIG.write().is_log = false;
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
                        class:"msg-log-span",
                        onclick: move |e|{
                            TEXTCONFIG.write().is_log = true;
                            e.stop_propagation();
                        },
                        "log",
                    }
                    if can_skip{
                        span{
                            class:"{auto_clicked} msg-auto-span",
                            onclick: move |e|{
                                let auto = !TEXTCONFIG.read().is_auto;
                                TEXTCONFIG.write().is_auto = auto;
                                e.stop_propagation();
                            },
                            "auto"
                        }
                        span{
                            class:"{skip_clicked} msg-skip-span",
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
                        class:"msg-close-span",
                        onclick: move |e|{
                            TEXTCONFIG.write().is_close = true;
                            e.stop_propagation();
                        },
                        "close"
                    }
                    span{
                        class:"msg-setting-span",
                        onclick: move |e|{
                            TEXTCONFIG.write().is_setting = true;
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
                div{class: "setting-close"}
                nav{
                    class: "setting-close",
                    onclick: move |e|{
                        TEXTCONFIG.write().is_setting = false;
                        e.stop_propagation();
                    },
                    "exit"
                }
            }

    }
}
