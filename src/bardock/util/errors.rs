pub type CliResult = Result<(), CliError>;

#[derive(Debug)]
pub struct CliError {
    pub error: Option<anyhow::Error>,
    pub exit_code: i32,
}

impl CliError {
    pub fn new(error: anyhow::Error, code: i32) -> CliError {
        CliError {
            error: Some(error),
            exit_code: code,
        }
    }

    // pub fn code(code: i32) -> CliError {
    //     CliError {
    //         error: None,
    //         exit_code: code,
    //     }
    // }
}

impl From<structopt::clap::Error> for CliError {
    fn from(err: structopt::clap::Error) -> CliError {
        let code = if err.use_stderr() { 1 } else { 0 };
        CliError::new(err.into(), code)
    }
}
