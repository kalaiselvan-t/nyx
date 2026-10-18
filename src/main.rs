mod cli;
mod config;
mod error;
mod llm;

use std::env;
use std::process::ExitCode;

use crate::cli::{ask, repl};
use crate::config::Config;
use crate::error::NyxError;
use crate::llm::Client;

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("error: {e}");
            e.exit_code()
        }
    }
}

fn run() -> Result<(), NyxError> {
    let config = Config::from_env()?;
    let client= Client::new(&config);

    match env::args().nth(1) {
        Some(prompt) => ask(&client, &prompt),
        None => repl(&client),
    }
}
