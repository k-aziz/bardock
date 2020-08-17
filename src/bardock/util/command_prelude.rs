pub use crate::{CliError, CliResult, Config, ops::NewOptions};
pub use structopt::clap::{AppSettings, Arg, ArgMatches};
use super::errors::BardockResult;
use std::path::PathBuf;

pub trait ArgMatchesExt {
    fn new_options(&self, config: &Config) -> BardockResult<NewOptions> {
        NewOptions::new(
            self.value_of_path("path", config).unwrap(),
            self._value_of("name").map(|s| s.to_string())
        )
    }

    fn value_of_path(&self, name: &str, config: &Config) -> Option<PathBuf> {
        self._value_of(name).map(|path| config.cwd().join(path))
    }

    fn _value_of(&self, name: &str) -> Option<&str>;

}

impl<'a> ArgMatchesExt for ArgMatches<'a> {
    fn _value_of(&self, name: &str) -> Option<&str> {
        self.value_of(name)
    }
}