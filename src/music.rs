use bevy::prelude::*;
use bevy_kira_audio::Audio;

pub fn music_system(assets: Res<AssetServer>, audio: Res<Audio>) {
    audio.play_looped(assets.load("music/dungeon_theme_1.wav"));
}
