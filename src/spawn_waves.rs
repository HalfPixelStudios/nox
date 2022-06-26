use bevy::{core::Stopwatch, prelude::*};
use bevy_inspector_egui::{Inspectable, InspectorPlugin};
use rand::seq::SliceRandom;

use super::enemy::{spawn_simple_enemy, spawn_simple_enemy_strong};

pub struct SpawnWavesPlugin;

type SpawnFunction = fn(cmd: &mut Commands, spawn_pos: Vec2) -> ();

struct WaveInfo {
    spawn_pool: Vec<SpawnFunction>,
    spawn_count: u32,
}

#[derive(Inspectable)]
pub struct WaveResource {
    pub wave_number: u8,
    spawns_left: u32,
    pub wave_ongoing: bool,
    #[inspectable(ignore)]
    pub wave_timer: Stopwatch,
    #[inspectable(ignore)]
    spawn_timer: Stopwatch,

    cooldown_period: f32,
    spawn_speed: f32,
    #[inspectable(ignore)]
    waves: Vec<WaveInfo>,
}

impl Default for WaveResource {
    fn default() -> Self {
        WaveResource {
            wave_number: 0,
            spawns_left: 0,
            wave_ongoing: false,
            wave_timer: Stopwatch::new(),
            spawn_timer: Stopwatch::new(),
            cooldown_period: 20.,
            spawn_speed: 1.,
            waves: vec![],
        }
    }
}

impl Plugin for SpawnWavesPlugin {
    fn build(&self, app: &mut App) {
        let waves = vec![
            WaveInfo {
                spawn_pool: vec![spawn_simple_enemy],
                spawn_count: 5,
            },
            WaveInfo {
                spawn_pool: vec![spawn_simple_enemy_strong],
                spawn_count: 10,
            },
        ];

        app.add_system(wave_system)
            // .add_plugin(InspectorPlugin::<WaveResource>::new())
            .add_system(wave_spawn_system)
            .insert_resource(WaveResource {
                cooldown_period: 5.,
                waves,
                ..default()
            });
    }
}

impl WaveResource {
    fn total_waves(&self) -> u8 {
        self.waves.len() as u8
    }
    fn current_wave(&self) -> &WaveInfo {
        // TODO: will die if we have zero waves defined
        if self.wave_number == 0 || self.total_waves() == 0 {
            panic!("attempting to access wave zero or no waves");
        }
        let wave_number = self.wave_number.min(self.total_waves()) as usize;
        self.waves.get(wave_number - 1).unwrap()
    }
}

fn wave_system(time: Res<Time>, mut res: ResMut<WaveResource>) {
    // start new wave
    if res.spawns_left == 0 && res.wave_timer.elapsed_secs() > res.cooldown_period {
        res.wave_timer.pause();
        res.wave_timer.reset();

        res.wave_number += 1;
        res.spawns_left = res.current_wave().spawn_count;
        res.wave_ongoing = true;
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
