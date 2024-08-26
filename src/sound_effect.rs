use std::{collections::HashMap, error::Error, io::Cursor};

use dioxus::signals::{GlobalSignal, Readable, Signal};
use kira::{
    manager::{AudioManager, AudioManagerSettings, DefaultBackend},
    sound::static_sound::StaticSoundData,
};
use tracing::info;

use crate::story_base::TEXTCONFIG;
pub static AUDIO_MANAGER: GlobalSignal<AudioManager> = Signal::global(|| {
    let settings = AudioManagerSettings::default();
    // let mut capercities = Capacities::default();
    // capercities.sound_capacity = CAPS;

    // settings.capacities = capercities;
    let manager: AudioManager<DefaultBackend> = AudioManager::new(settings).unwrap();
    manager
});
pub static SOUND_EFFECTS: GlobalSignal<HashMap<String, Vec<u8>>> =
    Signal::global(|| HashMap::new());
// const BASE_SAMPLE_RATE: f64 = 44100.0;
#[derive(Debug, PartialEq, Clone)]
pub struct SoundEffect {
    // pub context: AudioContext,
    pub base_sound: StaticSoundData,
    pub pitch: f64,
    pub speed: f64,
    pub volum: f64,
    pub is_rev: bool,
    pub reverb: f64,
}
impl SoundEffect {
    pub fn new(data: Vec<u8>) -> Result<Self, Box<dyn Error>> {
        let cursor = Cursor::new(data);
        let sound_data = StaticSoundData::from_cursor(cursor)?;

        Ok(SoundEffect {
            // context: sink,
            base_sound: sound_data,
            pitch: 1.,
            speed: 1.,
            volum: 1.,
            is_rev: false,
            reverb: 0.,
        })
    }

    pub fn base(mut self, data: Vec<u8>) -> Result<Self, Box<dyn Error>> {
        let cursor = Cursor::new(data);
        let sound_data = StaticSoundData::from_cursor(cursor)?;

        self.base_sound = sound_data;
        Ok(self)
    }

    // pub async fn from_url(url: &str) -> Result<Self, Box<dyn Error>> {
    //     let client = Client::new();
    //     let res = client.get(url).send().await?.bytes().await?;

    //     let cursor = Cursor::new(res);
    //     let sound_data = StaticSoundData::from_cursor(cursor)?;
    //     Ok(SoundEffect {
    //         // context: sink,
    //         base_sound: sound_data,
    //         pitch: 1.,
    //         speed: 1.,
    //         volum: 1.,
    //         streach: 1.,
    //         is_rev: false,
    //         reverb: 0.,
    //     })
    // }

    pub fn pitch(mut self, p: f64) -> Self {
        self.pitch = p;
        self
    }
    pub fn speed(mut self, s: f64) -> Self {
        self.speed = s;
        self
    }
    pub fn volum(mut self, v: f64) -> Self {
        self.volum = v;
        self
    }
    pub fn reverb(mut self, e: f64) -> Self {
        self.reverb = e;
        self
    }
    pub fn is_rev(mut self, rev: bool) -> Self {
        self.is_rev = rev;
        self
    }

    pub fn play(&self) -> Result<(), Box<dyn Error>> {
        let sound = self.base_sound.clone();
        let sound = sound.playback_rate(self.speed);
        let sound = sound.volume(self.volum * TEXTCONFIG.read().sound_volum);
        let sound = sound.reverse(self.is_rev);
        // let track = AUDIO_MANAGER.write().add_sub_track({
        //     let mut builder = TrackBuilder::new();
        //     if self.reverb != 0. {
        //         builder.add_effect(ReverbBuilder::new().damping(self.reverb));
        //     }
        //     // builder.add_effect(LfoBuilder::new().frequency(self.pitch));
        //     // builder.add_effect(CompressorBuilder::new().ratio(self.speed));
        //     // builder.add_effect();
        //     builder
        // })?;

        info!("{}", AUDIO_MANAGER.write().num_sounds());
        (*AUDIO_MANAGER.write()).play(sound)?;

        Ok(())
    }
}
