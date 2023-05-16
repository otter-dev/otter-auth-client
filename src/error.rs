use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
#[allow(clippy::enum_variant_names)]
pub enum Error {
    #[error("dirs could not find the home directory")]
    MissingHomeDirectory,

    #[error(transparent)]
    JsonError(#[from] serde_json::Error),

    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),

    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error(transparent)]
    PathError(#[from] core::convert::Infallible),
}
