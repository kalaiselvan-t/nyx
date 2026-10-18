use std::fmt::Formatter;
use std::process::ExitCode;

pub enum NyxError {
    MissingKey,
    Network(ureq::Error),
    BadResponse(String),
    IO(std::io::Error),
    ProfileNotFound(String),
    BadProfile(toml::de::Error),
    SessionError,
}

impl NyxError {
    pub fn exit_code(&self) -> ExitCode {
        match self {
            NyxError::SessionError
            | NyxError::Network(_)
            | NyxError::BadResponse(_)
            | NyxError::IO(_) => ExitCode::from(1),
            NyxError::MissingKey | NyxError::ProfileNotFound(_) | NyxError::BadProfile(_) => {
                ExitCode::from(2)
            }
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
            NyxError::ProfileNotFound(e) => write!(f, "profile not found: {e}"),
            NyxError::BadProfile(e) => write!(f, "invalid profile: {e}"),
            NyxError::SessionError => write!(f, "Session had error"),
        }
    }
}

impl From<ureq::Error> for NyxError {
    fn from(value: ureq::Error) -> Self {
        NyxError::Network(value)
    }
}

impl From<std::io::Error> for NyxError {
    fn from(value: std::io::Error) -> Self {
        NyxError::IO(value)
    }
}

impl From<toml::de::Error> for NyxError {
    fn from(value: toml::de::Error) -> Self {
        NyxError::BadProfile(value)
    }
}
