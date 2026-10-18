use serde_json::json;
use std::io::{IsTerminal, Write, stdin, stdout};
use std::{env, process};

use std::process::ExitCode;

// openrouter/free
// nvidia/nemotron-3-super-120b-a12b:free
// nvidia/nemotron-3-ultra-550b-a55b:free
const DEFAULT_MODEL: &str = "nvidia/nemotron-3-super-120b-a12b:free";

struct Config {
    api_key: String,
    model: String,
}

impl Config {
    fn from_env() -> Config {
        let api_key = match env::var("NYX_API_KEY") {
            Ok(key) => key,
            Err(_) => {
                eprintln!(
                    "Model provider key is not set! Set it with\n export NYX_API_KEY=<key>\n"
                );
                process::exit(2)
            }
        };
        let model = env::var("NYX_MODEL").unwrap_or_else(|_| DEFAULT_MODEL.to_string());
        Config { api_key, model }
    }
}

fn main() -> ExitCode {
    let config = Config::from_env();

    match env::args().nth(1) {
        Some(prompt) => ask(&prompt, &config),
        None => repl(&config),
    }
}

/// Core Functions
fn call(prompt: &str, config: &Config) -> serde_json::Value {
    const SYSTEM_PROMPT: &str = "You are nyx, a Linux command line assistant. You answer like terminal output - concise and precise. \
     Your output is printed raw in a terminal and may be piped to other programs. \
     Rules: \
        - Plain text only. No markdown: no code fences, no **bold**, no headings, no emoji. \
        - No ASCII art, banners or decoration. \
     ";

    let url: &str = "https://openrouter.ai/api/v1/chat/completions";
    let authorization: String = format!("Bearer {}", config.api_key);

    let data = json!({
        "model": config.model,
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

    let request = ureq_agent
        .post(url)
        .header("Authorization", authorization)
        .header("content-type", "application/json");

    let mut response = request.send_json(&data).unwrap();
    response.body_mut().read_json().unwrap()
}

fn repl(config: &Config) -> ExitCode {
    let mut failed = false;

    print_terminal_prompt();

    for line in stdin().lines() {
        let line = line.unwrap();
        let line = line.trim();

        match line {
            "" => {}
            "exit" => break,
            _ => {
                if ask(line, config) != ExitCode::SUCCESS {
                    failed = true;
                }
            }
        }
        print_terminal_prompt();
    }

    if failed {
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}

/// Helper Functions

fn ask(prompt: &str, config: &Config) -> ExitCode {
    let response: serde_json::Value = call(prompt, config);
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

/// Small helper functions
fn print_terminal_prompt() {
    // Prints the '> ' symbol if it is a terminal.
    if stdin().is_terminal() {
        print!("> ");
        stdout().flush().unwrap();
    }
}
