use crate::conversation::Conversation;
use crate::error::NyxError;
use crate::llm::Client;
use clap::Parser;
use std::io::{IsTerminal, Write, stdin, stdout};

#[derive(Parser)]
#[command(name = "nyx", about = "A Linux command line assistant")]
pub struct Args {
    /// Profile to load from ~/.config/nyx/profiles/<name>.toml
    #[arg(long, default_value = "default")]
    pub profile: String,

    /// Prompt; starts the REPL if omitted
    pub prompt: Option<String>,
}

pub fn repl(client: &Client) -> Result<(), NyxError> {
    let mut conversation = Conversation::new();
    let mut failed = false;

    print_terminal_prompt()?;

    for line in stdin().lines() {
        let line = line?;
        let line = line.trim();

        match line {
            "" => {}
            "exit" => break,
            "reset" => conversation.reset(),
            _ => match conversation.send(client, line) {
                Ok(msg) => println!("{}", msg.content),
                Err(e) => {
                    eprintln!("error: {e}");
                    failed = true;
                }
            },
        }
        print_terminal_prompt()?;
    }

    if failed {
        Err(NyxError::SessionError)
    } else {
        Ok(())
    }
}

pub fn ask(client: &Client, prompt: &str) -> Result<(), NyxError> {
    let mut conversation = Conversation::new();
    let reply = conversation.send(client, prompt)?;
    println!("{}", reply.content);
    Ok(())
}

fn print_terminal_prompt() -> Result<(), NyxError> {
    // Prints the '> ' symbol if it is a terminal.
    if stdin().is_terminal() {
        print!("> ");
        stdout().flush()?;
    }
    Ok(())
}
