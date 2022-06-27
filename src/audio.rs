use bevy::prelude::*;
use bevy_kira_audio::Audio;
use rand::{seq::SliceRandom, Rng};

pub struct PlaySoundEvent(pub String);

#[derive(Component, Default)]
pub struct SoundEmitter {
    pub hurt_sounds: Vec<String>,
    pub die_sounds: Vec<String>,
}

impl SoundEmitter {
    pub fn pick_hurt_sound(&self) -> Option<&String> {
        self.hurt_sounds.choose(&mut rand::thread_rng())
    }
    pub fn pick_die_sound(&self) -> Option<&String> {
        self.die_sounds.choose(&mut rand::thread_rng())
    }
}

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(bevy_kira_audio::AudioPlugin)
            .add_startup_system(music_system)
            .add_event::<PlaySoundEvent>()
            .add_system(play_sound_system);
    }
}

pub fn music_system(assets: Res<AssetServer>, audio: Res<Audio>) {
    audio.play_looped(assets.load("music/dungeon_theme_1.wav"));
}

pub fn play_sound(assets: &Res<AssetServer>, audio: &Res<Audio>, sfx_path: &str) {
    let path = format!("sfx/{}", sfx_path);
    audio.play(assets.load(&path));
}

pub fn play_random_sound(assets: &Res<AssetServer>, audio: &Res<Audio>, sfx_paths: Vec<&str>) {
    if sfx_paths.len() == 0 {
        return;
    }
    let sfx_path = sfx_paths.choose(&mut rand::thread_rng()).unwrap();
    let path = format!("sfx/{}", sfx_path);
    audio.play(assets.load(&path));
}

fn play_sound_system(
    assets: Res<AssetServer>,
    audio: Res<Audio>,
    mut sound_requests: EventReader<PlaySoundEvent>,
) {
    for event in sound_requests.iter() {
        play_sound(&assets, &audio, &event.0);
    }
}
