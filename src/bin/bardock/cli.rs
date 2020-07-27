use super::commands;
use bardock::{self, CliResult, Config};
use structopt::clap::ArgMatches;
use structopt::StructOpt;

// #[derive(Debug, StructOpt)]
// pub struct Test {
//     pub arg1: String,
//     pub arg2: Option<String>,
// }

// #[derive(Debug, StructOpt)]
// pub struct New {}

// #[derive(Debug, StructOpt)]
// pub enum Command {
//     New(New),
//     Test(Test)
// }

#[derive(Debug, StructOpt)]
pub enum Command {
    New { path: String},
    Test { arg1: String, arg2: Option<String> },
}

#[derive(Debug, StructOpt)]
#[structopt(name = "bardock", about = "python extension builder")]
pub struct Cli {
    #[structopt(subcommand)]
    command: Command,
}

pub fn main(config: &mut Config) -> CliResult {
    let clap_args = Cli::clap().get_matches();

    let (cmd, subcommand_args) = match clap_args.subcommand() {
        (cmd, Some(args)) => (cmd, args),
        _ => {
            // No subcommand provided.
            Cli::clap().print_help()?;
            return Ok(());
        }
    };

    execute_subcommand(config, cmd, subcommand_args)
}

fn execute_subcommand(config: &mut Config, cmd: &str, args: &ArgMatches) -> CliResult {
    if let Some(exec) = commands::builtin_exec(cmd) {
        return exec(config, args);
    }
    Ok(())
}
