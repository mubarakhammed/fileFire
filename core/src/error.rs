use thiserror::Error;

#[derive(Error, Debug)]
pub enum FilefireError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("PDF parsing error: {0}")]
    Pdf(String),

    #[error("Plugin error: {0}")]
    Plugin(String),

    #[error("Unsupported format: {0}")]
    UnsupportedFormat(String),

    #[error("Invalid document: {0}")]
    InvalidDocument(String),

    #[error("Annotation error: {0}")]
    Annotation(String),

    #[error("Metadata error: {0}")]
    Metadata(String),

    #[error("FFI error: {0}")]
    Ffi(String),

    #[error("Generic error: {0}")]
    Generic(String),
}

pub type Result<T> = std::result::Result<T, FilefireError>;
