use crate::command_prelude::*;


pub fn exec(args: &ArgMatches<'_>) -> CliResult {
    println!("do stuff");
    println!("{:?}", args.value_of("arg1"));

    Ok(())
}
