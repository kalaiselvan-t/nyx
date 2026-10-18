use serde_json::json;
use std::env;
use std::fmt::Formatter;
use std::io::{Error, IsTerminal, Write, stdin, stdout};
use std::process::ExitCode;

// openrouter/free
// nvidia/nemotron-3-super-120b-a12b:free
// nvidia/nemotron-3-ultra-550b-a55b:free
const DEFAULT_MODEL: &str = "nvidia/nemotron-3-super-120b-a12b:free";

enum NyxError {
    MissingKey,
    Network(ureq::Error),
    BadResponse(String),
    IO(std::io::Error),
    SessionError,
}

impl NyxError {
    fn exit_code(&self) -> ExitCode {
        match self {
            NyxError::SessionError
            | NyxError::Network(_)
            | NyxError::BadResponse(_)
            | NyxError::IO(_) => ExitCode::from(1),
            NyxError::MissingKey => ExitCode::from(2),
        }
    }
}

impl std::fmt::Display for NyxError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            NyxError::MissingKey => write!(
                f,
                "Model provider key is not set! Set it with\n export NYX_API_KEY=<key>"
            ),
            NyxError::Network(e) => write!(f, "Network error: {e}"),
            NyxError::BadResponse(msg) => write!(f, "Model provided a bad response: {msg}"),
            NyxError::IO(e) => write!(f, "IO error: {e}"),
            NyxError::SessionError => write!(f, "Session had error"),
        }
    }
}

impl From<ureq::Error> for NyxError {
    fn from(value: ureq::Error) -> Self {
        NyxError::Network(value)
    }
}

impl From<Error> for NyxError {
    fn from(value: Error) -> Self {
        NyxError::IO(value)
    }
}

struct Config {
    api_key: String,
    model: String,
}

impl Config {
    fn from_env() -> Result<Config, NyxError> {
        let api_key: String = match env::var("NYX_API_KEY") {
            Ok(key) => key,
            Err(_) => return Err(NyxError::MissingKey),
        };
        let model = env::var("NYX_MODEL").unwrap_or_else(|_| DEFAULT_MODEL.to_string());
        Ok(Config { api_key, model })
    }
}

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

/// Core Functions
fn call(prompt: &str, config: &Config) -> Result<String, NyxError> {
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

    let mut response = request.send_json(&data)?;
    let body: serde_json::Value = response.body_mut().read_json()?;

    match body["choices"][0]["message"]["content"].as_str() {
        Some(text) => Ok(text.to_string()),
        None => Err(NyxError::BadResponse(body.to_string())),
    }
}

fn repl(config: &Config) -> Result<(), NyxError> {
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

/// Helper Functions

fn ask(prompt: &str, config: &Config) -> Result<(), NyxError> {
    let response = call(prompt, config)?;
    println!("{}", response);
    Ok(())
}

/// Small helper functions
fn print_terminal_prompt() -> Result<(), NyxError> {
    // Prints the '> ' symbol if it is a terminal.
    if stdin().is_terminal() {
        print!("> ");
        stdout().flush()?;
    }
    Ok(())
}
