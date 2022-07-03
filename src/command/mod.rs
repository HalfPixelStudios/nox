use bevy::prelude::*;

pub struct CommandPlugin;

pub const PREFIX: &str = "/";

pub struct Command {
    pub name: String,
}

impl Plugin for CommandPlugin {
    fn build(&self, app: &mut App) {

    }
}


pub fn run_cmd(command_string: &str) {
    
}
