use crate::CliResult;
use std::env;
use thiserror::Error;

#[derive(Debug, Error)]
enum EnvironmentError {
    #[error("Failed to find `.env` file. Initialize it using the `example.env` as a reference.")]
    EnvironmentNotFound,
}

#[derive(Debug, Clone)]
pub struct Environment {
    pub log_level: Option<String>,
}

impl Environment {
    /// Init a new environment. Run at start of app to check for all required variables preventing
    /// runtime errors due to missing or incorrect environment variables.
    pub fn new() -> CliResult<Environment> {
        dotenvy::dotenv().map_err(|_e| EnvironmentError::EnvironmentNotFound)?;

        // Required Variable
        // let log_level =
        //     env::var("LOG_LEVEL").map_err(|_| dotenvy::Error::EnvVar(env::VarError::NotPresent))?;

        // Optional Variable
        let log_level = env::var("LOG_LEVEL").ok();

        Ok(Environment { log_level })
    }
}
