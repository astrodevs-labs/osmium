use thiserror::Error;

#[derive(Error, Debug)]
pub enum SlitherError {
    #[error("Error while runing slither")]
    SlitherError,
    #[error("Error while runing the slither command: {0}")]
    SlitherCommandError(#[from] std::io::Error),
    #[error("Error while parsing slither output: {0}")]
    SlitherParseError(#[from] serde_json::Error),
}
