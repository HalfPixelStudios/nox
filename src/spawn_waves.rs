use bevy::{core::Stopwatch, prelude::*};
use rand::seq::SliceRandom;

use super::enemy::spawn_simple_enemy;

pub struct SpawnWavesPlugin;

type SpawnFunction = fn(cmd: &mut Commands, spawn_pos: Vec2) -> ();

struct WaveInfo {
    spawn_pool: Vec<SpawnFunction>,
    spawn_count: u32,
}

struct WaveResource {
    wave_number: u8,
    spawns_left: u32,
    wave_ongoing: bool,
    wave_timer: Stopwatch,
    spawn_timer: Stopwatch,

    cooldown_period: f32,
    spawn_speed: f32,
    waves: Vec<WaveInfo>,
}

impl Plugin for SpawnWavesPlugin {
    fn build(&self, app: &mut App) {
        let waves = vec![WaveInfo {
            spawn_pool: vec![spawn_simple_enemy],
            spawn_count: 5,
        }];

        app.add_system(wave_system)
            .add_system(wave_spawn_system)
            .insert_resource(WaveResource {
                wave_number: 0,
                spawns_left: 0,
                wave_ongoing: false,
                wave_timer: Stopwatch::new(),
                spawn_timer: Stopwatch::new(),
                cooldown_period: 1.,
                spawn_speed: 1.,
                waves,
            });
    }
}

impl WaveResource {
    fn total_waves(&self) -> u8 {
        self.waves.len() as u8
    }
    fn current_wave(&self) -> &WaveInfo {
        // TODO: will die if we have zero waves defined
        let wave_number = self.wave_number.min(self.total_waves()) as usize;
        self.waves.get(wave_number).unwrap()
    }
}

fn wave_system(time: Res<Time>, mut res: ResMut<WaveResource>) {
    // start new wave
    if res.spawns_left == 0 && res.wave_timer.elapsed_secs() > res.cooldown_period {
        res.wave_timer.pause();
        res.wave_timer.reset();

        res.spawns_left = res.current_wave().spawn_count;

        res.wave_ongoing = true;
        res.wave_number += 1;
    }

    res.wave_timer.tick(time.delta());
}

fn wave_spawn_system(mut cmd: Commands, time: Res<Time>, mut res: ResMut<WaveResource>) {
    if res.wave_ongoing == false {
        return;
    }

    // spawn a new enemy
    if res.spawn_timer.elapsed_secs() > res.spawn_speed {
        res.spawn_timer.reset();
        spawn_enemy(&mut cmd, res.current_wave());
        res.spawns_left -= 1;
    }

    // end wave
    if res.spawns_left == 0 {
        res.wave_ongoing = false;
        res.wave_timer.unpause();
    }

    res.spawn_timer.tick(time.delta());
}

fn spawn_enemy(cmd: &mut Commands, current_wave: &WaveInfo) {
    let spawn_point: Vec2 = Vec2::new(0., 0.);

    let spawn_fn = current_wave.spawn_pool.choose(&mut rand::thread_rng());
    if spawn_fn.is_none() {
        return;
    }
    let spawn_fn = spawn_fn.unwrap();
    spawn_fn(cmd, spawn_point);
}
