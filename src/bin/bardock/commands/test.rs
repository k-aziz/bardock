use crate::command_prelude::*;
use bardock::Config;


pub fn exec(config: &mut Config, args: &ArgMatches<'_>) -> CliResult {
    log::debug!("do stuff");
    log::debug!("{:?}", args.value_of("arg1"));
    log::debug!("{:?}", config);

    Ok(())
}
