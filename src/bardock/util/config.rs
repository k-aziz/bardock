use std::{env, path::{Path, PathBuf}};
use super::errors::BardockResult;
use anyhow::Context;

#[derive(Debug)]
pub struct Config {
    cwd: PathBuf,
}

impl Config {
    pub fn new(cwd: PathBuf) -> Config {
        Config{
            cwd
        }
    }

    pub fn default() -> BardockResult<Config> {
        let cwd = env::current_dir().with_context(
            || "couldn't get the current directory of the process"
        )?;

        Ok(Config::new(cwd))
    }

    pub fn cwd(&self) -> &Path {
        &self.cwd
    }

}
