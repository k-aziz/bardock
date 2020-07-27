use crate::command_prelude::*;

use bardock::{Config, ops};

pub fn exec(config: &mut Config, args: &ArgMatches<'_>) -> CliResult {
    // todo: put this in options or config
    let path = args.value_of("path").unwrap();

    ops::new(&path, config)?;

    Ok(())
}
