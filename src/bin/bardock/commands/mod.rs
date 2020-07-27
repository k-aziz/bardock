use crate::command_prelude::*;
use bardock::Config;

pub fn builtin_exec(cmd: &str) -> Option<fn(&mut Config, &ArgMatches<'_>) -> CliResult> {
    let f = match cmd {
        "new" => new::exec,
        "test" => test::exec,
        &_ => return None
    };
    Some(f)
}

pub mod new;
pub mod test;
