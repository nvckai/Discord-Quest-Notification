use thiserror::Error;

/// Application error types
#[derive(Error, Debug, Clone)]
pub enum AppError {
    /// Configuration-related errors (missing or invalid config)
    #[error("Configuration error: {0}")]
    Config(String),
    
    /// HTTP request errors
    #[error("Request error: {0}")]
    Request(String),
    
    /// JSON parsing errors
    #[error("Parse error: {0}")]
    Parse(String),
}

impl From<reqwest::Error> for AppError {
    fn from(err: reqwest::Error) -> Self {
        AppError::Request(err.to_string())
    }
}

impl From<dotenvy::Error> for AppError {
    fn from(err: dotenvy::Error) -> Self {
        AppError::Config(err.to_string())
    }
}
