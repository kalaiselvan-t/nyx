mod cli;
mod config;
mod error;
mod llm;

use std::process::ExitCode;

use crate::cli::{Args, ask, repl};
use crate::config::Config;
use crate::error::NyxError;
use crate::llm::Client;
use clap::Parser;

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
    let args = Args::parse();
    let config = Config::load(&args.profile)?;
    let client = Client::new(&config);

    match args.prompt {
        Some(prompt) => ask(&client, &prompt),
        None => repl(&client),
    }
}
