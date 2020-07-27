use bardock::util::{config::Config, command_prelude};
mod cli;
mod commands;

fn main() {
    dotenv::dotenv().ok();
    
    // env_logger::init_from_env("BARDOCK_LOG");
    env_logger::init();

    // todo: setup config
    let mut config = match Config::default() {
        Ok(cfg) => cfg,
        Err(e) => bardock::exit_with_error(e.into())
    };

    match cli::main(&mut config) {
        Ok(()) => {},
        Err(e) => bardock::exit_with_error(e),
    };
}
