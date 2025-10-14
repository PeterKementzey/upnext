use std::fmt::Display;

#[derive(Debug)]
pub enum UpNextError {
    GenericError(String),
    IoError(std::io::Error),
    VlcError(String),
    VlcCommandNotFoundError,
    SchemaError(String),
    MissingSeries,
    SeriesAlreadyExists,
    SeriesOver,
    WrongEpisodeNumber,
    Unimplemented,
}

impl Display for UpNextError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UpNextError::GenericError(e) => write!(f, "Error: {e}"),
            UpNextError::IoError(e) => write!(f, "IO error: {e}"),
            UpNextError::VlcError(e) => write!(f, "VLC error: {e}"),
            UpNextError::VlcCommandNotFoundError => write!(f, "VLC command not found. Please ensure VLC is installed and in your PATH."),
            UpNextError::SchemaError(e) => write!(f, "Schema error: {e}"),
            UpNextError::MissingSeries => write!(f, "No series found for current working directory. Please run `{} init` first.", crate::APP_NAME),
            UpNextError::SeriesAlreadyExists => write!(f, "Current directory is already initialized."),
            UpNextError::SeriesOver => write!(f, "Season is over. No more episodes left in directory."),
            UpNextError::WrongEpisodeNumber => write!(f, "\nCanceled due to episode numbering out of sync."),
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

impl From<std::num::TryFromIntError> for UpNextError {
    fn from(e: std::num::TryFromIntError) -> Self {
        UpNextError::GenericError(format!("Could not convert integer: {e}"))
    }
}

impl<T> From<UpNextError> for Result<T> {
    fn from(e: UpNextError) -> Self {
        Err(e)
    }
}

pub type Result<T> = std::result::Result<T, UpNextError>;
