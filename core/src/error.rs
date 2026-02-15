use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RhodiError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Format error: {0}")]
    Format(String),

    #[error("Serialization error: {0}")]
    Serialization(String),

    #[error("Crypto error: {0}")]
    Crypto(String),

    #[error("Security violation: {0}")]
    Security(#[from] SecurityError),

    #[error("Extraction error: {0}")]
    Extraction(String),

    #[error("Verification failed: {0}")]
    Verification(String),

    #[error("Resolution error: {0}")]
    Resolution(String),
}

#[derive(Error, Debug)]
pub enum SecurityError {
    #[error("Path traversal detected: {path} is outside the allowed root {root}")]
    PathTraversal { path: PathBuf, root: PathBuf },

    #[error("Maximum recursion depth ({depth}) exceeded")]
    MaxRecursionDepth { depth: usize },

    #[error("Circular include detected: {path}")]
    CircularInclude { path: PathBuf },
}

pub type Result<T> = std::result::Result<T, RhodiError>;
