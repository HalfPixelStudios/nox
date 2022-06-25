use super::{app::run_app, config::AppState, error::BoxResult};
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

    run_app(app_state, fullscreen);

    Ok(())
}
