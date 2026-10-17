use std::{env, process};
use std::io::{stdin, stdout, IsTerminal, Write};
use serde_json::json;

use std::process::ExitCode;

fn main() -> ExitCode {
    let key = get_key();

    match env::args().nth(1) {
        Some(prompt) => ask(prompt, key),
        None => repl(key),
    }
}

/// Core Functions
fn call(prompt: String, key: String) -> serde_json::Value {

    // openrouter/free
    // nvidia/nemotron-3-super-120b-a12b:free
    // nvidia/nemotron-3-ultra-550b-a55b:free
    let model: String = "nvidia/nemotron-3-super-120b-a12b:free".to_string();
    const SYSTEM_PROMPT: &str = "You are nyx, a Linux command line assistant. You answer like terminal output - concise and precise. \
     Your output is printed raw in a terminal and may be piped to other programs. \
     Rules: \
        - Plain text only. No markdown: no code fences, no **bold**, no headings, no emoji. \
        - No ASCII art, banners or decoration. \
     ";

    let url: String = "https://openrouter.ai/api/v1/chat/completions".to_string();
    let authorization: String = format!("Bearer {key}");

    let data = json!({
        "model": model,
        "messages": [
            {
                "role": "system",
                "content": SYSTEM_PROMPT,
            },
            {
                "role": "user",
                "content": prompt,
            }
        ],
    });

    let ureq_agent: ureq::Agent = ureq::Agent::config_builder()
        .ip_family(ureq::config::IpFamily::Ipv4Only)
        .build()
        .into();

    let request = ureq_agent.post(url)
        .header("Authorization", authorization)
        .header("content-type", "application/json");

    let mut response = request.send_json(&data).unwrap();
    response.body_mut().read_json().unwrap()
}

fn repl(key: String) -> ExitCode {
    let mut failed = false;

    print_terminal_prompt();

    for line in stdin().lines() {
        let line = line.unwrap();
        let line = line.trim();

        match line {
            "" => {}
            "exit" => break,
            _ => {
                if ask(line.to_string(), key.clone()) != ExitCode::SUCCESS { failed = true;}
            }
        }
        print_terminal_prompt();
    }

    if failed { ExitCode::FAILURE } else { ExitCode::SUCCESS }
}

/// Helper Functions

fn ask(prompt: String, key: String) -> ExitCode {
    let response: serde_json::Value = call(prompt, key);
    match response["choices"][0]["message"]["content"].as_str() {
        Some(text) => {
            println!("{}", text);
            ExitCode::SUCCESS
        }
        None => {
            eprintln!("error: unexpected response from agent\n{}", response);
            ExitCode::FAILURE
        }
    }
}

fn get_key() -> String {
    match std::env::var("KEY") {
        Ok(key) => key,
        Err(_) => {
            eprintln!("Model provider key is not set! Set it with\n export KEY = <key>\n");
            process::exit(2);
        }
    }
}

/// Small helper functions
fn print_terminal_prompt() {
    // Prints the '> ' symbol if it is a terminal.
    if stdin().is_terminal() {
        print!("> ");
        stdout().flush().unwrap();
    }
}
