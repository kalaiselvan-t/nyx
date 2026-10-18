use std::io::{ stdin, stdout, IsTerminal, Write };
use crate::config::Config;
use crate::error::NyxError;
use crate::llm::call;

pub fn repl(config: &Config) -> Result<(), NyxError> {
    let mut failed = false;

    print_terminal_prompt()?;

    for line in stdin().lines() {
        let line = line?;
        let line = line.trim();

        match line {
            "" => {}
            "exit" => break,
            _ => {
                if let Err(e) = ask(line, config) {
                    eprintln!("error: {e}");
                    failed = true;
                }
            }
        }
        print_terminal_prompt()?;
    }

    if failed {
        Err(NyxError::SessionError)
    } else {
        Ok(())
    }
}

pub fn ask(prompt: &str, config: &Config) -> Result<(), NyxError> {
    let response = call(prompt, config)?;
    println!("{}", response);
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
