use std::fmt::Display;

#[derive(Debug)]
pub enum UpNextError {
    GenericError(String),
    IoError(std::io::Error),
    SchemaError(String),
    MissingSeries,
    SeriesAlreadyExists,
    Unimplemented,
}

impl Display for UpNextError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UpNextError::GenericError(e) => write!(f, "Error: {}", e),
            UpNextError::IoError(e) => write!(f, "IO error: {}", e),
            UpNextError::SchemaError(e) => write!(f, "Schema error: {}", e),
            UpNextError::MissingSeries => write!(f, "No series found. Please run `{} init` first.", crate::commands::APP_NAME),
            UpNextError::SeriesAlreadyExists => write!(f, "Current directory is already initialized."),
            UpNextError::Unimplemented => write!(f, "Unimplemented"),
        }
    }
}

impl From<std::io::Error> for UpNextError {
    fn from(e: std::io::Error) -> Self {
        UpNextError::IoError(e)
    }
}

impl From<toml_edit::TomlError> for UpNextError {
    fn from(e: toml_edit::TomlError) -> Self {
        UpNextError::SchemaError(e.to_string())
    }
}

impl<T> From<UpNextError> for Result<T> {
    fn from(e: UpNextError) -> Self {
        Err(e)
    }
}

pub type Result<T> = std::result::Result<T, UpNextError>;
