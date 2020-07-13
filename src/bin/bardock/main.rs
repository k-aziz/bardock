// use bardock::util::{self, command_prelude, CliResult};
use bardock::util::command_prelude;
mod cli;
mod commands;

// use crate::command_prelude::*;

fn main() {
    // todo: setup config

    match cli::main() {
        Ok(()) => {},
        Err(e) => bardock::exit_with_error(e),
    };
}
