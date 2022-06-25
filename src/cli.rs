use super::{app::run_app, error::BoxResult};
use argparse::{Cli, Command, Flag, FlagParse};

pub fn run_cli() -> BoxResult<()> {
    let cli = Cli {
        program_name: "nox",
        synopsis: "",
        root_command: Command {
            command_name: "run",
            desc: "run game",
            handler: handle_run,
            flags: vec![],
        },
        ..Default::default()
    };

    let args = std::env::args().collect();
    cli.run(&args)?;
    Ok(())
}

fn handle_run(flagparse: FlagParse) -> BoxResult<()> {
    run_app();

    Ok(())
}
