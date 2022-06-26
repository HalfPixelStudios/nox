use super::{
    app::{run_app, AppConfig},
    config::AppState,
    error::BoxResult,
};
use argparse::{Cli, Command, Flag, FlagParse};

pub fn run_cli() -> BoxResult<()> {
    let cli = Cli {
        program_name: "nox",
        synopsis: "",
        root_command: Command {
            command_name: "run",
            desc: "run game",
            handler: handle_run,
            flags: vec![
                Flag::new('s')
                    .long("appstate")
                    .desc("which app state to start in (mainmenu|ingame)")
                    .parameter(), // TOOD add default option to argparse
                Flag::new('f').long("fullscreen"),
                Flag::new('e').long("egui"),
                Flag::new('r').long("debug-render"),
                Flag::new('d')
                    .long("debug-mode")
                    .desc("enable all debug features"),
            ],
        },
        ..Default::default()
    };

    let args = std::env::args().collect();
    cli.run(&args)?;
    Ok(())
}

fn handle_run(flagparse: FlagParse) -> BoxResult<()> {
    let start_state = flagparse
        .get_flag_value::<String>('s')
        .unwrap_or(String::new());
    let app_state: AppState = match start_state.as_str() {
        "mainmenu" => AppState::MainMenu,
        "ingame" => AppState::InGame,
        _ => AppState::InGame, // default to in game
    };

    let fullscreen = flagparse.get_flag('f');
    let egui_enabled = flagparse.get_flag('e');
    let debug_render = flagparse.get_flag('r');
    let debug_mode = flagparse.get_flag('d');

    let mut config = AppConfig {
        app_state,
        fullscreen,
        egui_enabled,
        debug_render,
    };

    if debug_mode {
        config.egui_enabled = true;
        config.debug_render = true;
    }

    run_app(config);

    Ok(())
}
