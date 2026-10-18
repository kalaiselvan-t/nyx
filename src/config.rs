use crate::error::NyxError;
use serde::Deserialize;
use std::env;
use std::fs;
use std::io::ErrorKind;
use std::path::PathBuf;

// openrouter/free
// nvidia/nemotron-3-super-120b-a12b:free
// nvidia/nemotron-3-ultra-550b-a55b:free
const DEFAULT_MODEL: &str = "nvidia/nemotron-3-super-120b-a12b:free";
const DEFAULT_PROFILE: &str = "default";

#[derive(Deserialize)]
#[serde(default)]
struct Profile {
    model: String,
}

impl Default for Profile {
    fn default() -> Self {
        Profile {
            model: DEFAULT_MODEL.to_string(),
        }
    }
}

pub struct Config {
    pub api_key: String,
    pub model: String,
}

impl Config {
    pub fn load(profile_name: &str) -> Result<Config, NyxError> {
        let api_key = env::var("NYX_API_KEY").map_err(|_| NyxError::MissingKey)?;
        let profile = load_profile(profile_name)?;
        let model = env::var("NYX_MODEL").unwrap_or(profile.model);
        Ok(Config { api_key, model })
    }
}

fn profiles_dir() -> PathBuf {
    let config_home = match env::var("XDG_CONFIG_HOME") {
        Ok(dir) if !dir.is_empty() => PathBuf::from(dir),
        _ => PathBuf::from(env::var("HOME").unwrap_or_default()).join(".config"),
    };
    config_home.join("nyx").join("profiles")
}

fn load_profile(name: &str) -> Result<Profile, NyxError> {
    let path = profiles_dir().join(format!("{name}.toml"));

    match fs::read_to_string(&path) {
        Ok(text) => Ok(toml::from_str(&text)?),
        Err(e) if e.kind() == ErrorKind::NotFound && name == DEFAULT_PROFILE => {
            Ok(Profile::default())
        }
        Err(e) if e.kind() == ErrorKind::NotFound => {
            Err(NyxError::ProfileNotFound(path.display().to_string()))
        }
        Err(e) => Err(e.into()),
    }
}
