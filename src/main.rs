use nyx::cli::{Args, ask, repl};
use nyx::config::Config;
use nyx::error::NyxError;
use nyx::llm::Client;

use std::process::ExitCode;
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
