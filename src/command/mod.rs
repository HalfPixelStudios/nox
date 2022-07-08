mod error;

use std::collections::HashMap;
use error::*;
use bevy::prelude::*;

pub const PREFIX: &str = "/";

pub struct CommandPlugin;

impl Plugin for CommandPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(capture_input);
    }
}

struct CommandInputEvent {
    input: String
}

pub struct ExecuteCommandEvent {

}

pub struct Arg {
    pub arg_name: String,
}

pub struct Command {
    pub cmd_name: String,
    pub desc: String,
    pub args: Vec<Arg>,
}

pub struct CommandApp {
    pub cmds: Vec<Command>
}

pub struct ArgParse {

}

pub struct CommandParse {
    cmd_name: String,
    // args: HashMap<String, ArgParse>,
}

pub fn parse_command(command_app: &CommandApp, command_string: &str) -> Result<CommandParse> {

    let mut it = command_string.split_whitespace();

    let cmd_name = it.next().ok_or(Box::new(Error::InvalidCommand))?;

    // check that command is valid
    let cmd_def = command_app.cmds.iter().find(|c| cmd_name == c.cmd_name).ok_or(Error::InvalidCommand)?;

    return Ok(CommandParse {
        cmd_name: cmd_name.to_string(),
        // args: HashMap::new()
    })
}

fn capture_input() {

}
