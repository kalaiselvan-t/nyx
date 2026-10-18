use std::env;
use crate::error::NyxError;

// openrouter/free
// nvidia/nemotron-3-super-120b-a12b:free
// nvidia/nemotron-3-ultra-550b-a55b:free
const DEFAULT_MODEL: &str = "nvidia/nemotron-3-super-120b-a12b:free";

pub struct Config {
    pub api_key: String,
    pub model: String,
}

impl Config {
    pub fn from_env() -> Result<Config, NyxError> {
        let api_key: String = match env::var("NYX_API_KEY") {
            Ok(key) => key,
            Err(_) => return Err(NyxError::MissingKey),
        };
        let model = env::var("NYX_MODEL").unwrap_or_else(|_| DEFAULT_MODEL.to_string());
        Ok(Config { api_key, model })
    }
}