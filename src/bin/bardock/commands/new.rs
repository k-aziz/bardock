use crate::command_prelude::*;

use bardock::{Config, ops};

pub fn exec(config: &mut Config, args: &ArgMatches<'_>) -> CliResult {
    let opts = args.new_options(config)?;

    ops::new(&opts, config)?;

    Ok(())
}
