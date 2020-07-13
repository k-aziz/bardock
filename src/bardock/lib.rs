pub use crate::util::{CliError, CliResult};

use anyhow::Error;
use log::debug;

pub mod util;

pub fn exit_with_error(err: CliError) -> ! {
    debug!("exit_with_error; err={:?}", err);
    if let Some(ref err) = err.error {
        if let Some(clap_err) = err.downcast_ref::<structopt::clap::Error>() {
            clap_err.exit()
        }
    }

    let CliError { error, exit_code } = err;
    if let Some(error) = error {
        display_error(&error);
    }

    std::process::exit(exit_code)
}


pub fn display_error(err: &Error) {
    debug!("display_error; err={:?}", err);
}
