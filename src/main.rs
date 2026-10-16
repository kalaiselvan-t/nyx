use std::io::{IsTerminal, Write, stdin, stdout};
use std::process::ExitCode;

fn main() -> ExitCode{
    let interactive = stdin().is_terminal();
    let mut failed: bool = false;
    
    if interactive {
        print!("> ");
        stdout().flush().unwrap();
    }

    for line in stdin().lines() {
        let line = line.unwrap();
        let words: Vec<&str> = line.split_whitespace().collect();

        match words.as_slice() {
            [] => {},
            ["exit"] => break,
            ["echo", rest @ ..] => println!("{}", rest.join(" ")),
            ["help"] => println!("\nWelcome to Nyx! This your help guide\n\necho: Prints the text that follows this command\nhelp: Prints this help guide\n"),
            [cmd, ..] => {
                eprintln!("Unknown command: {cmd}");
                failed = true;
            }
        }

        if interactive {
            print!("> ");
            stdout().flush().unwrap();
        }
    }

    if failed {
        ExitCode::from(1)
    }
    else {
        ExitCode::SUCCESS
    }
}
