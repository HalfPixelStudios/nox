use bevy::prelude::*;
use bevy_kira_audio::Audio;

pub struct PlaySoundEvent(pub String);

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

fn play_sound_system(
    assets: Res<AssetServer>,
    audio: Res<Audio>,
    mut sound_requests: EventReader<PlaySoundEvent>,
) {
    for event in sound_requests.iter() {
        play_sound(&assets, &audio, &event.0);
    }
}
