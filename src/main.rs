mod error;
mod config;
mod llm;
mod cli;

use std::env;
use std::process::ExitCode;

use crate::config::Config;
use crate::error::NyxError;
use crate::cli::{ ask, repl };

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

    match env::args().nth(1) {
        Some(prompt) => ask(&prompt, &config),
        None => repl(&config),
    }
}